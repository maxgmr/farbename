XDG_DATA_HOME ?= ~/.local/share
PREFIX ?= /usr

default:
	cargo b --release

install-dryrun:
	$(info mkdir -p ${PREFIX}/local/bin)
	$(info cp ./target/release/farbename ${PREFIX}/local/bin)
	$(info mkdir -p ${XDG_DATA_HOME}/farbename)
	$(info cp ./colours.db ${XDG_DATA_HOME}/farbename)

install:
	sudo mkdir -p ${PREFIX}/local/bin
	sudo cp ./target/release/farbename ${PREFIX}/local/bin
	mkdir -p ${XDG_DATA_HOME}/farbename
	cp ./colours.db ${XDG_DATA_HOME}/farbename

clean-dryrun:
	$(info cargo clean)
	$(info rm ${PREFIX}/local/bin/farbename)
	$(info rm -r ${XDG_DATA_HOME}/farbename)

clean:
	cargo clean
	sudo rm ${PREFIX}/local/bin/farbename
	sudo rm -r ${XDG_DATA_HOME}/farbename
