# Project template for pico running rtic

![ci checks](https://github.com/adoble/pico-rtic-template/actions/workflows/ci_checks.yml/badge.svg)

This template is intended as a starting point for developing rp-pico based application using the [cortex-m-rtic](https://crates.io/crates/cortex-m-rtic) crate. It is based on [this rp2040 template](https://github.com/rp-rs/rp2040-project-template) and [this rtic example](https://github.com/rtic-rs/rtic-examples/blob/master/rtic_v1/rp-pico_local_initilzd_resources/src/main.rs).

It does the following:
- Blinks the rp-pico on-board led (GPIO 25) using a timer
- Processes a interrupt when GPIO 17 is pulled low (e.g with a push button)

It includes all of the `knurling-rs` tooling as showcased in https://github.com/knurling-rs/app-template (`defmt`, `defmt-rtt`, `panic-probe`, `flip-link`) to make development as easy as possible.

# Downloading

`probe-run` is configured as the default runner, so you can start the program with
```sh
cargo run --release
```

## Requirements 
  
- The standard Rust tooling (cargo, rustup) which you can install from https://rustup.rs/

- Toolchain support for the cortex-m0+ processors in the rp2040 (thumbv6m-none-eabi)

- flip-link - this allows you to detect stack-overflows on the first core, which is the only supported target for now.

- probe-run. Upstream support for RP2040 was added with version 0.3.1.

- A CMSIS-DAP probe. (J-Link and other probes will not work with probe-run)

  You can use a second Pico as a CMSIS-DAP debug probe by installing either of the following firmware on it:

  https://github.com/majbthrd/DapperMime/releases/download/20210225/raspberry_pi_pico-DapperMime.uf2

  https://raw.githubusercontent.com/9names/binary-bits/main/rust-dap-pico-ramexec-setclock.uf2

  More details on supported debug probes can be found in [debug_probes.md](debug_probes.md)


## Installation of development dependencies 

```sh
rustup target install thumbv6m-none-eabi
cargo install flip-link
# Suggested default 'runner'
cargo install probe-run
# If you want to use elf2uf2-rs instead of probe-run, instead do...
cargo install elf2uf2-rs --locked
```

## Running
  
For a debug build
```sh
cargo run
```
For a release build
```sh
cargo run --release
```

If you do not specify a DEFMT_LOG level, it will be set to `debug`.
That means `println!("")`, `info!("")` and `debug!("")` statements will be printed.
If you wish to override this, you can change it in `.cargo/config.toml` 
```toml
[env]
DEFMT_LOG = "off"
```
You can also set this inline (on Linux/MacOS)  
```sh
DEFMT_LOG=trace cargo run
```

or set the _environment variable_ so that it applies to every `cargo run` call that follows:
#### Linux/MacOS/unix
```sh
export DEFMT_LOG=trace
```

Setting the DEFMT_LOG level for the current session  
for bash
```sh
export DEFMT_LOG=trace
```

#### Windows
Windows users can only override DEFMT_LOG through `config.toml`
or by setting the environment variable as a separate step before calling `cargo run`
- cmd
```cmd
set DEFMT_LOG=trace
```
- powershell
```ps1
$Env:DEFMT_LOG = trace
```

```cmd
cargo run
```

