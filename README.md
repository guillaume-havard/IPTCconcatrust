# IPTC concatenate

Tool to concatenate several csv files and check their content.

## What will you need to compile

### If you are on linux(debian)

sudo apt install mingw-w64
rustup target add x86_64-pc-windows-gnu

build:
cargo build --release

build for windows:
cargo build --release --target x86_64-pc-windows-gnu


