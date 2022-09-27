use hypercore::PartialKeypair;

cfg_if::cfg_if! { if #[cfg(feature = "v10")] {
use anyhow::Result;
use async_std::net::TcpStream;
use async_std::sync::{Arc, Mutex};
use async_std::task;
use futures_lite::stream::StreamExt;
use hypercore::{Hypercore, Node, Proof, PublicKey, Signature, Storage};
use log::*;
use random_access_memory::RandomAccessMemory;
use random_access_storage::RandomAccess;
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::env;
use std::fmt::Debug;

use hypercore_protocol::schema::*;
use hypercore_protocol::{discovery_key, Channel, Event, Message, ProtocolBuilder};

mod util;
use util::{tcp_client, tcp_server};

fn main() {
    util::init_logger();
    if env::args().count() < 3 {
        usage();
    }
    let mode = env::args().nth(1).unwrap();
    let port = env::args().nth(2).unwrap();
    let address = format!("127.0.0.1:{}", port);

    let key = env::args().nth(3);
    let key: Option<[u8; 32]> = key.map_or(None, |key| {
        Some(
            hex::decode(key)
                .expect("Key has to be a hex string")
                .try_into()
                .expect("Key has to be a 32 byte hex string"),
        )
    });

    task::block_on(async move {
        let mut hypercore_store: HypercoreStore<RandomAccessMemory> = HypercoreStore::new();
        let storage = Storage::new_memory().await.unwrap();
        // Create a hypercore.
        let hypercore = if let Some(key) = key {
            let public_key = PublicKey::from_bytes(&key).unwrap();
            Hypercore::new_with_key_pair(storage, PartialKeypair{public: public_key, secret: None}).await.unwrap()
        } else {
            let mut hypercore = Hypercore::new(storage).await.unwrap();
            hypercore.append_batch(&[b"hi\n", b"ola\n", b"hello\n", b"mundo\n"]).await.unwrap();
            hypercore.append(b"world").await.unwrap();
            hypercore
        };
        info!("{} opened hypercore: {}", mode, hex::encode(hypercore.key_pair().public.as_bytes()));
        // Wrap it and add to the hypercore store.
        let hypercore_wrapper = HypercoreWrapper::from_memory_hypercore(hypercore);
        hypercore_store.add(hypercore_wrapper);
        let hypercore_store = Arc::new(hypercore_store);

        let result = match mode.as_ref() {
            "server" => tcp_server(address, onconnection, hypercore_store).await,
            "client" => tcp_client(address, onconnection, hypercore_store).await,
            _ => panic!(usage()),
        };
        util::log_if_error(&result);
    });
}

/// Print usage and exit.
fn usage() {
    println!("usage: cargo run --example hypercore -- [client|server] [port] [key]");
    std::process::exit(1);
}

// The onconnection handler is called for each incoming connection (if server)
// or once when connected (if client).
// Unfortunately, everything that touches the hypercore_store or a feed has to be generic
// at the moment.
async fn onconnection<T: 'static>(
    stream: TcpStream,
    is_initiator: bool,
    hypercore_store: Arc<HypercoreStore<T>>,
) -> Result<()>
where
    T: RandomAccess<Error = Box<dyn std::error::Error + Send + Sync>> + Debug + Send,
{
    info!("onconnection, initiator: {}", is_initiator);
    let mut protocol = ProtocolBuilder::new(is_initiator).connect(stream);
    info!("protocol created, polling for next()");
    while let Some(event) = protocol.next().await {
        let event = event?;
        info!("protocol event {:?}", event);
        match event {
            Event::Handshake(_) => {
                if is_initiator {
                    for feed in hypercore_store.hypercores.values() {
                        protocol.open(feed.key().clone()).await?;
                    }
                }
            }
            Event::DiscoveryKey(dkey) => {
                if let Some(feed) = hypercore_store.get(&dkey) {
                    protocol.open(feed.key().clone()).await?;
                }
            }
            Event::Channel(channel) => {
                if let Some(feed) = hypercore_store.get(channel.discovery_key()) {
                    feed.onpeer(channel);
                }
            }
            Event::Close(_dkey) => {}
            _ => {}
        }
    }
    Ok(())
}



/// A container for hypercores.
#[derive(Debug)]
struct HypercoreStore<T>
where
    T: RandomAccess<Error = Box<dyn std::error::Error + Send + Sync>> + Debug + Send,
{
    hypercores: HashMap<String, Arc<HypercoreWrapper<T>>>,
}
impl<T> HypercoreStore<T>
where
    T: RandomAccess<Error = Box<dyn std::error::Error + Send + Sync>> + Debug + Send,
{
    pub fn new() -> Self {
        let hypercores = HashMap::new();
        Self { hypercores }
    }

    pub fn add(&mut self, hypercore: HypercoreWrapper<T>) {
        let hdkey = hex::encode(&hypercore.discovery_key);
        self.hypercores.insert(hdkey, Arc::new(hypercore));
    }

    pub fn get(&self, discovery_key: &[u8; 32]) -> Option<&Arc<HypercoreWrapper<T>>> {
        let hdkey = hex::encode(discovery_key);
        self.hypercores.get(&hdkey)
    }
}

/// A Hypercore is a single unit of replication, an append-only log.
#[derive(Debug, Clone)]
struct HypercoreWrapper<T>
where
    T: RandomAccess<Error = Box<dyn std::error::Error + Send + Sync>> + Debug + Send,
{
    discovery_key: [u8; 32],
    key: [u8; 32],
    hypercore: Arc<Mutex<Hypercore<T>>>,
}

impl HypercoreWrapper<RandomAccessMemory> {
    pub fn from_memory_hypercore(hypercore: Hypercore<RandomAccessMemory>) -> Self {
        let key = hypercore.key_pair().public.to_bytes();
        HypercoreWrapper {
            key,
            discovery_key: discovery_key(&key),
            hypercore: Arc::new(Mutex::new(hypercore)),
        }
    }
}

impl<T> HypercoreWrapper<T>
where
    T: RandomAccess<Error = Box<dyn std::error::Error + Send + Sync>> + Debug + Send + 'static,
{
    pub fn key(&self) -> &[u8; 32] {
        &self.key
    }

    pub fn onpeer(&self, mut channel: Channel) {
        let mut state = PeerState::default();
        let mut hypercore = self.hypercore.clone();
        task::spawn(async move {
            let msg = Want {
                start: 0,
                length: None,
            };
            channel.send(Message::Want(msg)).await.unwrap();
            while let Some(message) = channel.next().await {
                let result = onmessage(&mut hypercore, &mut state, &mut channel, message).await;
                if let Err(e) = result {
                    error!("protocol error: {}", e);
                    break;
                }
            }
        });
    }
}

/// A PeerState stores the head seq of the remote.
/// This would have a bitfield to support sparse sync in the actual impl.
#[derive(Debug)]
struct PeerState {
    remote_head: Option<u64>,
}
impl Default for PeerState {
    fn default() -> Self {
        PeerState { remote_head: None }
    }
}

async fn onmessage<T>(
    hypercore: &mut Arc<Mutex<Hypercore<T>>>,
    state: &mut PeerState,
    channel: &mut Channel,
    message: Message,
) -> Result<()>
where
    T: RandomAccess<Error = Box<dyn std::error::Error + Send + Sync>> + Debug + Send,
{
    match message {
        // TODO
        _ => {}
    };
    Ok(())
}

// cfg_if
} else { fn main() {} } }
