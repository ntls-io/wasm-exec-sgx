# NTC - WASM component

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

1. Make sure the `docker` daemon is running, otherwise start it with
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
3. Run `make all` to compile the entire project.
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

---

## Alternative Instructions (11/2022)

1. Ensure you have compatible hardware and enable sgx in the bios and libraries 

2. Install prereq libraries and Docker

sudo apt-get install make cmake clang autoconf libtool

### Install docker
https://docs.docker.com/engine/install/ubuntu/
In order to run the scripts as a non-root user, follow the docker-postinstall
https://docs.docker.com/engine/install/linux-postinstall/

### Install Rust
Install rustup as described at https://rustup.rs and set nightly as the default toolchain
rustup install nightly
rustup default nightly


#### Install the driver, linux-sgx-driver, download and install as above 
Install SGX driver
Follow https://github.com/intel/linux-sgx-driver
**OR** 
wget https://download.01.org/intel-sgx/sgx-linux/2.11/distro/ubuntu18.04-server/sgx_linux_x64_driver_2.6.0_b0a445b.bin
sudo ./sgx_linux_x64_driver_2.6.0_b0a445b.bin

Then check that the folder exists:
ls /dev/isgx

### Install the Linux SGX SDK

Source: https://github.com/apache/incubator-teaclave-sgx-sdk/tree/a6a172e652b4db4eaa17e4faa078fda8922abdd0

git clone https://github.com/apache/incubator-teaclave-sgx-sdk.git

sudo docker pull baiduxlab/sgx-rust:1804-1.1.3

#### Replace the teaclave path with the right one
TEACLAVE_PATH="/home/ubuntu/incubator-teaclave-sgx-sdk"
sudo docker run -v {TEACLAVE_PATH}:/root/sgx -ti --device /dev/isgx baiduxlab/sgx-rust:1804-1.1.3

**OR** to run in simulation mode
docker run -v /home/jbochenek/code/incubator-teaclave-sgx-sdk:/root/sgx -ti baiduxlab/sgx-rust:2004-1.1.3

#### Set aesm libraries and start the service EVERY TIME 
LD_LIBRARY_PATH=/opt/intel/sgx-aesm-service/aesm/ /opt/intel/sgx-aesm-service/aesm/aesm_service &

#### Run the sample code
root@docker:~/sgx/samplecode/helloworld# make
root@docker:~/sgx/samplecode/helloworld# cd bin
root@docker:~/sgx/samplecode/helloworld/bin# ./app
