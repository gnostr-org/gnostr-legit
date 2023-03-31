act:
	@act -vbr  -W .github/workflows/automate.yml #-P ubuntu-latest=node

act-rust-1_67:## 	docker rust image
	@act -vr  -W .github/workflows/rust-cross.yml #-P ubuntu-latest=node
act-rust-1_68:## 	docker rust image
	@act -vr  -W .github/workflows/rust-cross.yml -P ubuntu-latest=rust:1.68
act-rust-1_69:## 	docker rust image
	@act -vr  -W .github/workflows/rust-cross.yml -P ubuntu-latest=rust:1.69