# rust-bluepill
Alternates LEDs LD3 and LD4 on the STM32F407VGT6 discovery board every second.

## Pre-requisites
**Update rust**

    rustup update

**Install target**

    rustup target install thumbv7m-none-eabi

**Install flash tool (For ST-Link V2 and clones)**

    cargo install cargo-flash

**Get the code**

    git clone https://github.com/mmmstew/rust-stm32f407-blinky.git

**Jump into the folder**

    cd rust-stm32f407-blinky


## Usage
**Build the project**

    cargo build --release

**Flash to target**

    cargo flash --chip STM32F407VGTx --connect-under-reset --release

**If updating linker script or things Cargo might not notice, it can be helpful to follow up with**

    cargo clean
