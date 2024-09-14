all: build

build:
	./install.sh

run: src/*.rs
	cargo r

watch: src/*.rs
	cargo watch -x 'run'
