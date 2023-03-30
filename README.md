#### create root folder
```shell
cargo new cc1 --vcs=none
```

#### create folder cms/app
copy src and all Cargo.toml/lock to app folder

#### create Cargo.toml
```text
[workspace]

members = [
    "cmd/app",
]
```

#####
````text
cargo new internal --lib --vcs=none
````


##### Makefile
```text
build:
	@cargo build

clean:
	@cargo clean
	docker stop $$(docker ps -aq) || true
	docker rm $$(docker ps -aq) || true
	docker rmi hworld || true
	docker rmi $$(docker images -f "dangling=true" -q ) || true

TESTS = ""
test:
	@cargo test $(TESTS) --offline --lib -- --color=always --nocapture

docs: build
	@cargo doc --no-deps

style-check:
	@rustup component add rustfmt 2> /dev/null
	cargo fmt --all -- --check

lint:
	@rustup component add clippy 2> /dev/null
	cargo clippy --all-targets --all-features -- -D warnings

dev:
	cargo run

alpine:
	@sudo docker build -t rust-alpine -f ./alpine/Dockerfile .

m1:
	cargo build
	cross build --target x86_64-unknown-linux-musl --release

.PHONY: build test docs style-check lint alpine m1, arm

```