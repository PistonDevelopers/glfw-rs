#!/bin/sh

make lib 1>&2 # cargo-lite wants stdout
echo "cargo-lite: artifacts"
find build/lib -type f
