.PHONY: lambda clean deploy

BIN=target/x86_64-unknown-linux-musl/release/aws-lambda-rust-benchmark

$(BIN): src/main.rs Cargo.toml
	cargo build --target x86_64-unknown-linux-musl --release

lambda: $(BIN)
	mkdir -p lambda
	cp $(BIN) lambda/bootstrap

deploy: lambda
	cdk deploy

clean:
	rm -r target
	rm -r lambda