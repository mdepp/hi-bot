PI_TARGET = aarch64-unknown-linux-gnu
SERVER_HOST = server

.phony: build
build:
	cargo build --release

.phony: clean
clean:
	cargo clean

.phony: build-pi
build-pi:
	cross build --target $(PI_TARGET) --release 

.phony: deploy
deploy: build-pi
	ssh $(SERVER_HOST) 'mkdir -p ~/bin/hi-bot/'
	scp target/$(PI_TARGET)/release/hi-bot $(SERVER_HOST):~/bin/hi-bot/hi-bot
	scp .env $(SERVER_HOST):~/bin/hi-bot/.env
