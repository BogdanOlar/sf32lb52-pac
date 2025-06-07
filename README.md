# sf32lb52-pac
SF32LB52 Peripheral Access Crate

The code is generated automatically from a (modified) [vendor-supplied SVD](https://raw.githubusercontent.com/OpenSiFli/SiFli-SDK/refs/heads/main/tools/svd_external/SF32LB52X/SF32LB52x.svd) file, using [svd2rust](https://docs.rs/svd2rust) v0.35.0.

The `.svd` file has been modified to:
- derive peripherals from each other (e.g. derive `I2C2` and `I2C3` from `I2C1`, etc)
- remove all kinds of "reserved" registers and fields

in order to minimize the amount of generated code.

## Generate the PAC:

```sh
svd2rust -i SF32LB52x.svd -c svd2rust.toml
rm -rf src
form -i lib.rs -o src/ && rm lib.rs
cargo +nightly fmt
```
OR just run

```sh
./regenerate.sh
```