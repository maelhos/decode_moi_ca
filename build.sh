#!/bin/bash

RUSTFLAGS="-C target-cpu=native" cargo build --release --bin decode_moi_ca
cp target/release/decode_moi_ca .