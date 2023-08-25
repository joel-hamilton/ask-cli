.PHONY: build-all build-linux build-mac

build-linux:
	rm -f ./releases/openai-x86_64-unknown-linux-gnu
	cross build --target x86_64-unknown-linux-gnu --release
	cp ./target/x86_64-unknown-linux-gnu/release/openai ./releases/openai-x86_64-unknown-linux-gnu


# this will only work on mac
build-mac:
	rm -f ./releases/openai-aarch64-apple-darwin
	cargo build --release --target aarch64-apple-darwin
	cp ./target/aarch64-apple-darwin/release/openai ./releases/openai-aarch64-apple-darwin

build-all:
	make -j 2 build-linux build-mac
