PROJECT_NAME=popemodem

compile:
	@docker run --rm -it \
		-v $(PWD):/drone/src \
		-w /drone/src \
			joseluisq/rust-linux-darwin-builder:1.65.0 \
				make cross-compile
.PHONY: compile

cross-compile:
	@echo
	@echo "1. Cross compiling example..."
	@rustc -vV
	@echo
	@echo "2. Compiling application (linux-musl x86_64)..."
	@cargo build --manifest-path=./Cargo.toml --release --target x86_64-unknown-linux-musl
	@du -sh tests/hello-world/target/x86_64-unknown-linux-musl/release/helloworld
	@echo
	@echo "3. Compiling application (apple-darwin x86_64)..."
	@cargo build --manifest-path=./Cargo.toml --release --target x86_64-apple-darwin
	@du -sh ./target/x86_64-apple-darwin/release/$(PROJECT_NAME)
.PHONY: cross-compile

