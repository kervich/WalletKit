export MACOSX_DEPLOYMENT_TARGET ?= 15.0

.PHONY: all clean

all: spm

clean:
		@cargo clean
		$(RM) -r generated
		$(RM) -r WalletKit
framework:
		@cargo build --target x86_64-apple-darwin --release
		@cargo build --target aarch64-apple-darwin --release
		@mkdir -p target/universal-macos/release
		@lipo target/x86_64-apple-darwin/release/libsui.a target/aarch64-apple-darwin/release/libsui.a -create -output target/universal-macos/release/libsui.a
spm:
		@cargo swift package --name WalletKit -p macos --release
