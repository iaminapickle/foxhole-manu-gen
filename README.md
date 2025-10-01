# foxhole-manu-gen

[WIP] Generates optimal batches for use in Foxhole manufacturing.

For a more rigorous problem statement, read *FoxholeManuGen.pdf*.

## Installation - Build from Source
foxhole-manu-gen is written in [Rust](https://rust-lang.org/) and will generally track the latest version but should run on anything after 1.88.0 inclusive.

```bash
git clone https://github.com/iaminapickle/foxhole-manu-gen
cd foxhole-manu-gen
cargo build -- release
```

The binary executable will then be `/target/release/fmg.exe`.

## Usage
```
Usage: fmg.exe [OPTIONS]

Options:
    -o, --output               Enable output files
    -p, --path <PATH>          Output file path
    -l, --output-batch-long    Show full item names in output
    -h, --help                 Print help
```