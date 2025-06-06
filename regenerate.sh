#!/bin/bash

svd2rust -i SF32LB52x.svd -c svd2rust.toml
rm -rf src
form -i lib.rs -o src/ && rm lib.rs
cargo +nightly fmt
