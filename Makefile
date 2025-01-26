default:
	cargo b --release

install:
	sudo mkdir -p /usr/local/bin/
	sudo cp ./target/release/farbename /usr/local/bin/

clean:
	cargo clean
	sudo rm /usr/local/bin/farbename
