# Build the project in debug mode (for testing purposes)
build:
	cargo b

# Build the project in release mode
# This replicates how Vim works (e.g., `vim .` or `vim <filename>`)
# The binary 'comfy' is built into ./target/release/comfy
build_release:
	cargo build --release

# Install the release binary by moving it to /usr/local/bin
# This allows you to run 'comfy' from anywhere in the terminal
install:
	sudo mv ./target/release/comfy /usr/local/bin/

# Build in release mode and install the binary
bi: build_release install

# Clean the build
clean:
	cargo clean
