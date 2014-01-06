# File: Makefile
# Author: Phillip Walters
# Description:
# 	Builds glfw-rs

# Sources
LIB_SRC=$(shell find src/glfw -type f -name '*.rs')
EXAMPLES_SRC=$(shell find src/examples -type f -name '*.rs')

.SILENT:

.PHONY: all clean

all: .build_examples
	echo "Done!"

.build_lib: .setup_lib
	echo "Building glfw-rs..."
	rustc --out-dir lib src/glfw/lib.rs
	touch .build_lib

.setup_lib:
	mkdir -p lib
	touch .setup_lib

.build_examples: .build_lib .setup_examples ${EXAMPLES_SRC}
	echo "Building examples..."
	$(foreach file, ${EXAMPLES_SRC}, rustc --out-dir bin -L lib $(file);)
	touch .build_examples

.setup_examples:
	mkdir -p bin
	touch .setup_examples

clean:
	find . -type f -name '.build_*' | xargs rm -f
	rm -f lib/libglfw*
	echo "Cleaned"
