# IPTC concatenate

Tool to concatenate several csv files and check their content.

## What will you need to compile

### If you are on linux(debian)

```bash
# To cross-compile to Windows
sudo apt install mingw-w64
rustup target add x86_64-pc-windows-gnu
```

build:
```bash
cargo build --release
```

build for windows:
```bash
cargo build --release --target x86_64-pc-windows-gnu
```

## Usage

```bash
cargo run <Directory of csv file>
```

the output file will be called ``./output.csv``.

### Exemple of result for check

```bash
CSV files concatenated successfully!
Line 3
        * Sup category is 'FNAC', it should be 'MQB - Iconotheque'
StringRecord(["2", "2.545 Mio", "3200 DPI", " 2_01", "FNAC", "voiture", "evian"])

Line 4
        * Field 'toto' is empty
StringRecord(["3", "3.5 Mio", "3200 DPI", " 3_01", "MQB - Iconotheque", "", "evian"])

Line 5
        * Size '1.9 Mio' is below the minimum '2 Mio'
StringRecord(["4", "1.9 Mio", "3200 DPI", " 4_01", "MQB - Iconotheque", "ecrevisse", "evian"])

Line 6
        * Object name '99_01' does not match filename '5'
StringRecord(["5", "2.9 Mio", "3200 DPI", " 99_01", "MQB - Iconotheque", "pantoufle", "evian"])

Line 7
        * Source is 'vittel', it should be 'evian'
StringRecord(["6", "2.5 Mio", "3200 DPI", " 6_01", "MQB - Iconotheque", "licorne", "vittel"])

Line 8
        * Width is '1600 DPI', it should be '3200 DPI'
StringRecord(["7", "2.5 Mio", "1600 DPI", " 7_01", "MQB - Iconotheque", "Ane", "evian"])

Line 9
        * Size '5.5 Mio' is above the maximum '4 Mio'
        * Object name '88_01' does not match filename '8'
StringRecord(["8", "5.5 Mio", "3200 DPI", " 88_01", "MQB - Iconotheque", "poufsoufle", "evian"])

CSV file validated successfully!
```



