# sf32lb52-pac
SF32LB52 Peripheral Access Crate

The code is generated automatically from a vendor-supplied SVD file, using [svd2rust](https://docs.rs/svd2rust) v0.35.0:

```sh
svd2rust -i EFM32PG1B.svd -c svd2rust.toml
rm -rf src
form -i lib.rs -o src/ && rm lib.rs
cargo +nightly fmt
```