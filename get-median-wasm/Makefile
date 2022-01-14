
# TODO: Environment setup like rustup target add wasm32-wasi

SRC_WASM = ./
BIN = ../bin/
APP_WASM = get_median.wasm
BIN_WASM = $(addprefix $(BIN), $(APP_WASM))
all: $(BIN_WASM)

$(BIN_WASM):
	@echo "\033[32mStarting cargo to build the wasm binary...\033[0m"
	@cd $(SRC_WASM) && cargo build --target wasm32-unknown-unknown --release
	@cp $(SRC_WASM)/target/wasm32-unknown-unknown/release/get_median_wasm.wasm $(BIN)/$(APP_WASM)

clean:
	@cargo clean
	@rm -f $(BIN_WASM)
