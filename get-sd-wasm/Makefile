
SRC_WASM = ./
SRC_RST = ./src/
BIN = ../bin/
APP_WASM = get_sd_wasm.wasm
BIN_WASM = $(addprefix $(BIN),$(APP_WASM))
FILES_RUST_F= $(wildcard $(SRC_RST)*.rs) # Wildcard function used, no need to specify the rust files. Safe as we don't compile the rust files with the makefile.
TOML = Cargo.toml
all: $(BIN_WASM)

$(BIN_WASM): $(FILES_RUST_F) $(TOML)
	@echo "\033[32mStarting cargo to build the SD int wasm binary...\033[0m"
	@cd $(SRC_WASM) && cargo build --target wasm32-unknown-unknown --release
	@cp $(SRC_WASM)/target/wasm32-unknown-unknown/release/get_sd_wasm.wasm $(BIN)/$(APP_WASM)

clean:
	@cargo clean
	@rm -f $(BIN_WASM)
