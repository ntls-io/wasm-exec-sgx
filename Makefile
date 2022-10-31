# Dummy makefile, will call the host and enclave makefile when requested.

CC ?= clang
CXX ?= clang++

SRC_U = app/
SRC_T = enclave/
SRC_WASM_MED_INT= get-median-int-wasm/
SRC_WASM_MED_FLOAT= get-median-float-wasm/
SRC_WASM_MEAN_INT= get-mean-int-wasm/
SRC_WASM_MEAN_FLOAT= get-mean-float-wasm/
SRC_WASM_SD_INT= get-sd-int-wasm/
SRC_WASM_SD_FLOAT= get-sd-float-wasm/
SRC_WASM_APPEND= wasm-append/

# Compilation process, will call the appropriate makefiles.

all: host enclave wasm

host:
	@echo "\033[32mRequest to compile the host part...\033[0m"
	@make -C $(SRC_U)

enclave:
	@echo "\033[32mRequest to compile the enclave part...\033[0m"
	@make -C $(SRC_T)

wasm:
	@echo "\033[32mRequest to compile the wasm part...\033[0m"
	@make -C $(SRC_WASM_MED_INT)
	@make -C $(SRC_WASM_MED_FLOAT)
	@make -C $(SRC_WASM_MEAN_INT)
	@make -C $(SRC_WASM_MEAN_FLOAT)
	@make -C $(SRC_WASM_SD_INT)
	@make -C $(SRC_WASM_SD_FLOAT)
	@make -C $(SRC_WASM_APPEND)

clean:
	@make -C $(SRC_U) clean
	@make -C $(SRC_T) clean
	@make -C $(SRC_WASM_MED_INT) clean
	@make -C $(SRC_WASM_MED_FLOAT) clean
	@make -C $(SRC_WASM_MEAN_INT) clean
	@make -C $(SRC_WASM_MEAN_FLOAT) clean
	@make -C $(SRC_WASM_SD_INT) clean
	@make -C $(SRC_WASM_SD_FLOAT) clean
	@make -C $(SRC_WASM_APPEND) clean

fclean:
	@make -C $(SRC_U) fclean
	@make -C $(SRC_T) fclean

clean_host:
	@make -C $(SRC_U) clean

clean_enclave:
	@make -C $(SRC_T) clean

fclean_host:
	@make -C $(SRC_U) fclean

fclean_enclave:
	@make -C $(SRC_T) fclean

re_host: fclean_host host

re_enclave: fclean_enclave enclave

re: fclean all

# Dummy rules to let make know that those rules are not files.

.PHONY: host enclave clean clean_host clean_enclave fclean_host fclean_enclave fclean re re_host re_enclave
