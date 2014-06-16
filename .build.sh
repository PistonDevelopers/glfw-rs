#!/bin/sh

MAKE=make
if [ "$OS" == "Windows_NT" ]; then
	MAKE=mingw32-make
fi

$MAKE link
echo "made"
rustc --out-dir=$CARGO_OUT_DIR $CARGO_RUSTFLAGS src/lib/lib.rs

