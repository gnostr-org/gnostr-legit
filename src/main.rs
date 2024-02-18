#![allow(unused)]
#![allow(dead_code)]
extern crate chrono;
use chrono::offset::Utc;
use chrono::DateTime;
use std::process::Command;
//use std::time::SystemTime;
use std::any::type_name;
use std::convert::TryInto;
use std::env;
use std::io::Result;
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
//use std::mem::size_of;
use argparse::{ArgumentParser, Store};
use git2::*;
use gitminer::Gitminer;
use pad::{Alignment, PadStr};
use sha2::{Digest, Sha256};
use std::{io, thread};

use std::fs;
use std::path::Path;
use std::path::PathBuf; //for get_current_dir
use std::process;

mod gitminer;
mod repo;
mod worker;

//fn type_of<T>(_: T) -> &'static str {
//    type_name::<T>()
//}

fn get_epoch_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

fn convert_to_u32(v: usize) -> Option<i8> {
    if v > (std::i8::MAX as i32).try_into().unwrap() {
        None
    } else {
        Some(v as i8)
    }
}

fn get_current_working_dir() -> std::io::Result<PathBuf> {
    env::current_dir()
}

#[cfg(debug_assertions)]
fn example() {
    //println!("Debugging enabled");
    //println!("cwd={:?}",get_current_working_dir());
}

#[cfg(not(debug_assertions))]
fn example() {
    //println!("Debugging disabled");
    //println!("cwd={:?}",get_current_working_dir());
}

pub fn path_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

fn get_ref() {
    #[allow(dead_code)]

    let _git_file =

        //Path::new("../.git").is_file();
        Path::new(".git").is_file();

        //if _git_file true
        //git now we assume gnostr-legit is a submodule
        //to the parent repo
        //TODO: handle deeper nested submodule cases
        //TODO: make module is_submodule

    if _git_file {

        println!(".git is_file {}", _git_file);
        //gitdir: ../.git/modules/gnostr-legit

        const NAME: Option<&str> = option_env!("CARGO_PKG_NAME");
        const REF_MODULE: &str = include_str!("../.git");
        println!("REF_MODULE = {:?}", REF_MODULE);

        let v_ref_module: Vec<&str> = REF_MODULE.split(' ').collect();
        for _part in &v_ref_module {
            if REF_MODULE == "gitdir: ../.git/modules/gnostr-legit" {
                println!("REF_MODULE = {:?}", REF_MODULE);
                println!("{}/{}", NAME.unwrap_or("unknown"), REF_MODULE);
            }
            println!("&v_ref_module[0] = {}", &v_ref_module[0]);
            println!("&v_ref_module[1] = {}", &v_ref_module[1]);
            println!("{}/{}", NAME.unwrap_or("unknown"), &v_ref_module[1]);
            println!("{}", _part);
            //process::exit(0);
        }

        if REF_MODULE == "gitdir: ../.git/modules/gnostr-legit" {
            println!("REF_MODULE = {:?}", REF_MODULE);
            println!("{}/{}", NAME.unwrap_or("unknown"), REF_MODULE);
        }
    } else {
        println!(".git is_file {}", _git_file);
    } // end if _git_fileA
      //
    let _git_dir =
        //Path::new("../.git").is_dir();
        Path::new(".git").is_dir();
    if _git_dir {
        println!(".git is_dir {}", _git_dir);
    } else {
        println!(".git is_dir {}", _git_dir);
    }

    let ref_heads_master = Path::new("../.git/refs/heads/master").exists();
    println!("{}", ref_heads_master);
    let git_head = Path::new("../.git/HEAD").exists();
    println!("{}", git_head);

    //if ref_heads_master{
    //    //gitdir: ../.git/modules/gnostr-legit
    //    //println!("{}", Path::new("/etc/hosts").exists());
    const NAME: Option<&str> = option_env!("CARGO_PKG_NAME");
    const REF_MODULE: &str = include_str!("../.git");
    if REF_MODULE == "gitdir: ../.git/modules/gnostr-legit" {
        println!("REF_MODULE = {:?}", REF_MODULE);
        println!("{}/{}", NAME.unwrap_or("unknown"), REF_MODULE);
    }

    //const REF_MASTER: &str = include_str!("../.git/refs/heads/master");
    //}
    //if git_head{
    //    //gitdir: ../.git/modules/gnostr-legit
    //const REF: &str = include_str!("../.git/HEAD");
    //}
}

fn main() -> io::Result<()> {
    #[allow(clippy::if_same_then_else)]
    if cfg!(debug_assertions) {
        //println!("Debugging enabled");
    } else {
        //println!("Debugging disabled");
    }
    get_ref();

    #[cfg(debug_assertions)]
    //println!("Debugging enabled");
    #[cfg(not(debug_assertions))]
    //println!("Debugging disabled");
    example();

    let start = time::get_time();
    let epoch = get_epoch_ms();
    //println!("{}", epoch);
    let system_time = SystemTime::now();

    let datetime: DateTime<Utc> = system_time.into();
    //println!("{}", datetime.format("%d/%m/%Y %T/%s"));
    //println!("{}", datetime.format("%d/%m/%Y %T"));

    let cwd = get_current_working_dir();
    //#[cfg(debug_assertions)]
    //println!("Debugging enabled");
    //println!("{:#?}", cwd);
    let state = repo::state();
    //println!("{:#?}", state);
    //
    let repo_root = std::env::args().nth(1).unwrap_or(".".to_string());
    //println!("repo_root={:?}", repo_root.as_str());
    let repo = Repository::open(repo_root.as_str()).expect("Couldn't open repository");
    //println!("{} state={:?}", repo.path().display(), repo.state());
    //println!("state={:?}", repo.state());

    //println!("clean {:?}", repo.state());
    #[allow(clippy::if_same_then_else)]
    let repo_path = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "cd"])
            .output()
            .expect("failed to execute process")
    } else if cfg!(target_os = "macos") {
        Command::new("sh")
            .arg("-c")
            .arg("pwd")
            .output()
            .expect("failed to execute process")
    } else if cfg!(target_os = "linux") {
        Command::new("sh")
            .arg("-c")
            .arg("pwd")
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("pwd")
            .output()
            .expect("failed to execute process")
    };

    let path = String::from_utf8(repo_path.stdout)
        .map_err(|non_utf8| String::from_utf8_lossy(non_utf8.as_bytes()).into_owned())
        .unwrap();
    //println!("path={:?}", path);

    //#!/bin/bash
    //declare -a RELAYS
    //function gnostr-get-relays(){

    //RELAYS=$(curl  'https://api.nostr.watch/v1/online' 2>/dev/null |
    //    sed -e 's/[{}]/''/g' |
    //    sed -e 's/\[/''/g' |
    //    sed -e 's/\]/''/g' |
    //    sed -e 's/"//g' |
    //    awk -v k="text" '{n=split($0,a,","); for (i=1; i<=n; i++) print a[i]}') 2>/dev/null

    //echo $RELAYS
    //}
    //gnostr-get-relays

    //#!/bin/bash
    //gnostr-git config --global --replace-all gnostr.relays "$(gnostr-get-relays)" #&& git config -l | grep gnostr.relays
    #[allow(clippy::if_same_then_else)]
    let set_relays = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "gnostr-set-relays"])
            .output()
            .expect("try:\ngnostr-git config -l | grep gnostr.relays")
    } else if cfg!(target_os = "macos") {
        Command::new("sh")
            .arg("-c")
            .arg("gnostr-set-relays")
            .output()
            .expect("try:\ngnostr-git config -l | grep gnostr.relays")
    } else if cfg!(target_os = "linux") {
        Command::new("sh")
            .arg("-c")
            .arg("gnostr-set-relays")
            .output()
            .expect("try:\ngnostr-git config -l | grep gnostr.relays")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("gnostr-set-relays")
            .output()
            .expect("try:\ngnostr-git config -l | grep gnostr.relays")
    };

    let count = thread::available_parallelism()?.get();
    assert!(count >= 1_usize);
    //println!("{}={}", type_of(count), (count as i32));
    //println!("{}={}", type_of(count), (count as i64));
    //let mut hasher = Sha256::new();
    //hasher.update(pwd);
    //// `update` can be called repeatedly and is generic over `AsRef<[u8]>`
    //hasher.update("String data");
    //// Note that calling `finalize()` consumes hasher
    //let hash = hasher.finalize();
    ////println!("Binary hash: {:?}", hash);
    //println!("hash: {:?}", hash);
    //println!("sha256 before write: {:x}", hash);
    //println!("sha256 before write: {:X}", hash);

    let now = SystemTime::now();

    //// we sleep for 2 seconds
    //sleep(Duration::new(2, 0));
    // match now.elapsed() {
    //    Ok(elapsed) => {
    //        // it prints '2'
    //        println!("{}", elapsed.as_secs());
    //    }
    //    Err(e) => {
    //        // an error occurred!
    //        println!("Error: {e:?}");
    //    }
    //}

    #[allow(clippy::if_same_then_else)]
    let get_pwd = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "echo %cd%"])
            .output()
            .expect("failed to execute process")
    } else if cfg!(target_os = "macos") {
        Command::new("sh")
            .arg("-c")
            .arg("echo ${PWD##*/}")
            .output()
            .expect("failed to execute process")
    } else if cfg!(target_os = "linux") {
        Command::new("sh")
            .arg("-c")
            .arg("echo ${PWD##*/}")
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("echo ${PWD##*/}")
            .output()
            .expect("failed to execute process")
    };

    let pwd = String::from_utf8(get_pwd.stdout)
        .map_err(|non_utf8| String::from_utf8_lossy(non_utf8.as_bytes()).into_owned())
        .unwrap();
    //println!("pwd={}", pwd);
    let mut hasher = Sha256::new();
    hasher.update(pwd.clone());
    //sha256sum <(echo gnostr-legit)
    let pwd_hash: String = format!("{:x}", hasher.finalize());
    //println!("pwd_hash={:?}", pwd_hash);

    #[allow(clippy::if_same_then_else)]
    let gnostr_weeble = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "gnostr-weeble || echo weeble"])
            .output()
            .expect("failed to execute process")
    } else if cfg!(target_os = "macos") {
        Command::new("sh")
            .arg("-c")
            .arg("gnostr-weeble 2>/tmp/gnostr-legit.log || echo weeble")
            .output()
            .expect("failed to execute process")
    } else if cfg!(target_os = "linux") {
        Command::new("sh")
            .arg("-c")
            .arg("gnostr-weeble 2>/tmp/gnostr-legit.log || echo weeble")
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("gnostr-weeble 2>/tmp/gnostr-legit.log || echo weeble")
            .output()
            .expect("failed to execute process")
    };

    let weeble = String::from_utf8(gnostr_weeble.stdout)
        .map_err(|non_utf8| String::from_utf8_lossy(non_utf8.as_bytes()).into_owned())
        .unwrap();

    //assert_eq!(weeble.is_empty(), true); // a)
    //
    //println!("weeble={}", weeble);

    #[allow(clippy::if_same_then_else)]
    let gnostr_wobble = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "gnostr-wobble"])
            .output()
            .expect("failed to execute process")
    } else if cfg!(target_os = "macos") {
        Command::new("sh")
            .arg("-c")
            .arg("gnostr-wobble || echo wobble")
            .output()
            .expect("failed to execute process")
    } else if cfg!(target_os = "linux") {
        Command::new("sh")
            .arg("-c")
            .arg("gnostr-wobble || echo wobble")
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("gnostr-wobble || echo wobble")
            .output()
            .expect("failed to execute process")
    };

    let wobble = String::from_utf8(gnostr_wobble.stdout)
        .map_err(|non_utf8| String::from_utf8_lossy(non_utf8.as_bytes()).into_owned())
        .unwrap();
    //println!("wobble={}", wobble);
    #[allow(clippy::if_same_then_else)]
    let gnostr_blockheight = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "gnostr-blockheight"])
            .output()
            .expect("failed to execute process")
    } else if cfg!(target_os = "macos") {
        Command::new("sh")
            .arg("-c")
            .arg("gnostr-blockheight || echo blockheight")
            .output()
            .expect("failed to execute process")
    } else if cfg!(target_os = "linux") {
        Command::new("sh")
            .arg("-c")
            .arg("gnostr-blockheight || echo blockheight")
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("gnostr-blockheight || echo blockheight")
            .output()
            .expect("failed to execute process")
    };

    let blockheight = String::from_utf8(gnostr_blockheight.stdout)
        .map_err(|non_utf8| String::from_utf8_lossy(non_utf8.as_bytes()).into_owned())
        .unwrap();
    //println!("blockheight={}", blockheight);

    let path = env::current_dir()?;

    //println!("The current directory is {}", path.display());

    let mut opts = gitminer::Options {
        threads: count.try_into().unwrap(),
        target: "00000".to_string(), //default 00000
        //gnostr:##:nonce
        //part of the gnostr protocol
        //src/worker.rs adds the nonce
        pwd_hash: pwd_hash.clone(),
        message: pwd,
        //message: message,
        //message: count.to_string(),
        //repo:    ".".to_string(),
        repo: path.as_path().display().to_string(),
        timestamp: time::now(),
        weeble,
        wobble,
        blockheight,
        //.duration_since(SystemTime::UNIX_EPOCH)
    };

    parse_args_or_exit(&mut opts);

    let mut miner = match Gitminer::new(opts) {
        Ok(m) => m,
        Err(e) => {
            panic!("Failed to start git miner: {}", e);
        }
    };

    let hash = match miner.mine() {
        Ok(s) => s,
        Err(e) => {
            panic!("Failed to generate commit: {}", e);
        }
    };

    let mut hasher = Sha256::new();
    hasher.update(&hash);
    // `update` can be called repeatedly and is generic over `AsRef<[u8]>`
    //hasher.update("String data");
    // Note that calling `finalize()` consumes hasher
    //let gnostr_sec = hasher.finalize();
    let gnostr_sec: String = format!("{:X}", hasher.finalize());
    //println!("Binary hash: {:?}", hash);
    //println!("hash before: {:?}", hash);
    //println!("hash after pad: {:?}", hash);
    //println!("&hash before: {:?}", &hash);
    //println!("&hash after pad: {:?}", &hash);
    //println!("gnostr_sec before pad: {:?}", gnostr_sec);
    //println!("gnostr_sec after pad: {:?}", gnostr_sec.pad(64, '0', Alignment::Right, true));
    //println!("&gnostr_sec before pad: {:?}", &gnostr_sec);
    //println!("&gnostr_sec after pad: {:?}", &gnostr_sec.pad(64, '0', Alignment::Right, true));

    //let s = "12345".pad(64, '0', Alignment::Right, true);
    //println!("s: {:?}", s);
    // echo "000000b64a065760e5441bf47f0571cb690b28fc" | openssl dgst -sha256 | sed 's/SHA2-256(stdin)= //g'
    //
    //
    //shell test
    let touch = Command::new("sh")
        .args(["-c", "touch ", &hash])
        .output()
        .expect("failed to execute process");
    let touch_event = String::from_utf8(touch.stdout)
        .map_err(|non_utf8| String::from_utf8_lossy(non_utf8.as_bytes()).into_owned())
        .unwrap();
    let cat = Command::new("sh")
        .args(["-c", "touch ", &hash])
        .output()
        .expect("failed to execute process");
    let cat_event = String::from_utf8(cat.stdout)
        .map_err(|non_utf8| String::from_utf8_lossy(non_utf8.as_bytes()).into_owned())
        .unwrap();
    //shell test
    //git rev-parse --verify HEAD
    #[allow(clippy::if_same_then_else)]
    let event = if cfg!(target_os = "windows") {
        Command::new("cmd")
                .args(["/C", "gnostr --sec $(gnostr-sha256 $(gnostr-weeble || echo)) -t gnostr --tag weeble $(gnostr-weeble || echo weeble) --tag wobble $(gnostr-wobble || echo wobble) --tag blockheight $(gnostr-blockheight || echo blockheight) --content \"$(gnostr-git diff HEAD~1 || gnostr-git diff)\" "])
                .output()
                .expect("failed to execute process")
    } else if cfg!(target_os = "macos") {
        Command::new("sh")
                .args(["-c", "gnostr --sec $(gnostr-sha256 $(gnostr-weeble || echo)) -t gnostr --tag weeble $(gnostr-weeble || echo weeble) --tag wobble $(gnostr-wobble || echo wobble) --tag blockheight $(gnostr-blockheight || echo blockheight) --content \"$(gnostr-git show HEAD)\" "])
                .output()
                .expect("failed to execute process")
    } else if cfg!(target_os = "linux") {
        Command::new("sh")
                .args(["-c", "gnostr --sec $(gnostr-sha256 $(gnostr-weeble || echo)) -t gnostr --tag weeble $(gnostr-weeble || echo weeble) --tag wobble $(gnostr-wobble || echo wobble) --tag blockheight $(gnostr-blockheight || echo blockheight) --content \"$(gnostr-git diff HEAD~1 || gnostr-git diff)\" "])
                .output()
                .expect("failed to execute process")
    } else {
        Command::new("sh")
                .args(["-c", "gnostr --sec $(gnostr-sha256 $(gnostr-weeble || echo)) -t gnostr --tag weeble $(gnostr-weeble || echo weeble) --tag wobble $(gnostr-wobble || echo wobble) --tag blockheight $(gnostr-blockheight || echo blockheight) --content \"$(gnostr-git diff HEAD~1 || gnostr-git diff)\" "])
                .output()
                .expect("failed to execute process")
    };

    let gnostr_event = String::from_utf8(event.stdout)
        .map_err(|non_utf8| String::from_utf8_lossy(non_utf8.as_bytes()).into_owned())
        .unwrap();

    //assert...
    //echo gnostr|openssl dgst -sha256 | sed 's/SHA2-256(stdin)= //g'

    //gnostr-legit must only return a sha256 generated by the
    //recent commit hash
    //to enable nested commands
    //REF:
    //gnostr --hash $(gnostr legit . -p 00000 -m "git rev-parse --verify HEAD")
    //gnostr --sec $(gnostr --hash $(gnostr legit . -p 00000 -m "git rev-parse --verify HEAD"))
    //Example:
    //gnostr --sec $(gnostr --hash $(gnostr legit . -p 00000 -m "#gnostr will exist!")) --envelope --content "$(gnostr-git log -n 1)" | gnostr-cat -u wss://relay.damus.io
    //
    //
    //
    let duration = time::get_time() - start;
    //println!("Success! Generated commit {} in {} seconds", hash, duration.num_seconds());
    println!("{}", gnostr_event);
    Ok(())
}

fn parse_args_or_exit(opts: &mut gitminer::Options) {
    let mut ap = ArgumentParser::new();
    ap.set_description("Generate git commit sha with a custom prefix");
    ap.stop_on_first_argument(false);

    //ap.refer(&mut opts.repo)
    //    //.add_argument("repository-path", Store, "Path to your git repository (required)");
    //    .add_argument("repository-path", Store, "Path to your git repository");
    //    //.required();
    ap.refer(&mut opts.repo)
        .add_argument("repository-path", Store, "Path to your git repository");

    ap.refer(&mut opts.target).add_option(
        &["-p", "--prefix"],
        Store,
        "Desired commit prefix (required)",
    );
    //.required();

    ap.refer(&mut opts.threads).add_option(
        &["-t", "--threads"],
        Store,
        "Number of worker threads to use (default 8)",
    );

    ap.refer(&mut opts.message).add_option(
        &["-m", "--message"],
        Store,
        "Commit message to use (required)",
    );
    //.required();

    //ap.refer(&mut opts.timestamp)
    //    .add_option(&["--timestamp"], Store, "Commit timestamp to use (default now)");

    ap.parse_args_or_exit();
}
