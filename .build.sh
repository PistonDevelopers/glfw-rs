#!/bin/sh

make link
echo "made"
rustc --out-dir=$CARGO_OUT_DIR $CARGO_RUSTFLAGS src/lib/lib.rs
