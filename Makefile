all: clean build

NAME = "gritty"

clean:
	rm -rf target
build:
	cargo build --release
debug:
	cargo build
install:
	cp target/release/$(NAME) /usr/local/bin/$(NAME)
