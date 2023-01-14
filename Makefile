CURRENT_DIR := $(shell pwd)

.PHONY: build-server
build-server:
	cd rawmad-server && cargo build --release 

.PHONY: build-client
build-client:
	cd rawmad-client && cargo build --release

.PHONY: build
build: build-server build-client

$(CURRENT_DIR)/target/release/rawmad-client: build-client
	RUST_LOG=debug $(CURRENT_DIR)/target/release/rawmad-client

$(CURRENT_DIR)/target/release/rawmad-server: build-server
	RUST_LOG=debug $(CURRENT_DIR)/target/release/rawmad-server

.PHONY: run-client
run-client: $(CURRENT_DIR)/target/release/rawmad-client

.PHONY: run-server
run-server: $(CURRENT_DIR)/target/release/rawmad-server

run: run-client run-server



