# Rusty GBA dev test

Just a test project displaying and testing the capabilities of the [Rusty GBA library](https://github.com/litchipi/rusty_gbadev).

Provides the code, but also the build system to create a **.gba** file from scratch, using indications
of the awesome [gba crate](https://crates.io/crates/gba).

Can be used as a template to simply start a new project with Rusty GBA.

## Usage

``` bash
# Launch the built ROM
cargo make run

# Only create the ROM
cargo make create_rom

# Only build the code
cargo make build

# Only compresses the assets
# Will build the assets compressor if not present
cargo make assets

# Perform tests
cargo make test

# Deletes all the generated contents
cargo make reset
```

## Dependencies
In order to build this project, install the `nightly` rust channel:

``` bash
rustup install nightly
rustup +nightly component add rust-src
```

And install the `arm-none-eabi` toolchain, as well as some tools to build the complete toolchains,
example for Debian-based distros:
``` bash
cargo install gbafix
sudo apt install binutils-arm-none-eabi g++
```

You will also need [cargo-make](https://github.com/sagiegurari/cargo-make), and the `mgba` emulator:
``` bash
sudo apt install mgba-sdl
```

## Credits
- Assets compression taken from [gba-lz77 repository](https://github.com/lunasorcery/gba-lz77)
(forked to ensure it always stays up and compatible)

- Based on [the gba lib for Rust](https://github.com/rust-console/gba)
