#!/bin/sh

make lib 1>&2 # cargo-lite wants stdout
echo "cargo-lite: artifacts"
find lib -type f
