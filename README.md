# NTC - Execution Enclave and Service Development Repo

This repo serves as the development repo for the Exeuction Service and Execution Enclave for NTC.
The `app` directoy represents the execution service. 
The `enclave` directory represents the execution enclave. 
The `get-**-wasm` files represent rust code binaries.
The `wasmi-impl` represents a wasm binary compiler to convert and run the rust binaries to wasm code. 

## Installation

### Requirements

- Operating system:
    - Recommended: Ubuntu 20.04 (LTS)
- Packages:
    - `make`
    - `clang` (recommended) or `gcc`
    - `cmake`
    - `autoconf`
    - `libtool`
- Snaps:
    - `docker`
- Rust toolchain:
    - See [#rust-ecosystem]

### Rust ecosystem

Install `rustup` as described at [rustup.rs](https://rustup.rs/).  If you did
not explicitly select `nightly` as your default toolchain then do so now
```
$ rustup install nightly
```
followed by
```
$ rustup default nightly
```

You will now have the `cargo` build tool, as well as the nigtly `rustc` compiler
installed on your system and may now continue setting up your environment.

### Environment

**NOTE:** If you opted to install `gcc` as your compiler, make sure you run
```
export CC=gcc; export CXX=g++
```
Otherwise you may safely continue.

The Rust and Intel SGX SDKs need to be installed and the
relevant environment variables need to be set.  In order to facilitate this, we
use the convenience scripts provided at [rust-sgx-sdk-env].

1. Make sure docker is installed and the `docker` daemon is running, otherwise install following these steps -  [docker_install], once installed you can check the docker is up by running this code:
   ```
   $ systemctl start snap.docker.dockerd
   ```
2. In order to run the scripts as a non-root user, follow the
   [docker-postinstall](post-installation instructions) set out in the Docker
   documentation (note, in particular, that a restart may be necessary).
3. Clone the repository at [rust-sgx-sdk-env]
   ```
   $ git clone https://github.com/PiDelport/rust-sgx-sdk-dev-env
   ```
   and `cd` into it.
4. Run the latest "prepare" script:
   ```
   $ ./prepare-1.1.14-intel-2.15.1.sh
   ```
5. Finally, assuming `bash` is the current shell, source the environment file in
   the top level of the repository:
   ```
   $ source environment
   ```

### Instructions

1. Before proceeding, make sure your [environment is set up](#environment)
   properly.
2. Clone the project repository
    ```
    $ git clone https://github.com/ntls-io/wasm-exec-sgx
    ```
   and `cd` into it.
3. Run `make` to compile the entire project.
4. To run the main application, change to bin/ and execute the following:
     ```
    ./app
    ```
5. In order to test the provided Wasm binary, change the current directory to
   the `wasmi-impl` subdirectory and execute the following:
    ```
    cargo test
    ```

[docker-postinstall]: https://docs.docker.com/engine/install/linux-postinstall/
[rust-sgx-sdk-env]: https://github.com/PiDelport/rust-sgx-sdk-dev-env
[docker-install]: https://docs.docker.com/engine/install/ubuntu/
