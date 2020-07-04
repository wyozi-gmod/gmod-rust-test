## Rust for GMod 

See https://github.com/SpiralP/rust-glua-sys for a more recent take

### Why
- Easier cross platform library creation (good for eg. file IO or using libraries that are in cargo)

### Why not
- Don't use it for things that need to interact with source sdk. Rust does not play nice with C++

### How

__Windows:__
1. `cargo build --target=i686-pc-windows-msvc` (because srcds is 32-bit)

Note: you might need to install 32 bit libc or whatever from rustup.

__Linux:__
1. `cargo build` (you might need to set target to 32- bit if this does not work)
