#!/bin/sh

rustc --out-dir=$CARGO_OUT_DIR $CARGO_RUSTFLAGS -C link-args="$(sh etc/glfw-link-args.sh)" src/lib/lib.rs
