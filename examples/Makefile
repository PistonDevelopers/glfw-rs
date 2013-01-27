RS_FLAGS	:= -L ../lib
# RS_FLAGS	:= -g -L ../lib 		# causes "error: internal compiler error: create_function: unexpected sort of node"
BUILD_DIR	:= bin


all: callbacks modes time title version window
	
builddir:
	@mkdir -p $(BUILD_DIR)
	
callbacks: builddir
	rustc callbacks.rs $(RS_FLAGS) --out-dir $(BUILD_DIR)
	
modes: builddir
	rustc modes.rs $(RS_FLAGS) --out-dir $(BUILD_DIR)
	
time: builddir
	rustc time.rs $(RS_FLAGS) --out-dir $(BUILD_DIR)
	
title: builddir
	rustc title.rs $(RS_FLAGS) --out-dir $(BUILD_DIR)
	
version: builddir
	rustc version.rs $(RS_FLAGS) --out-dir $(BUILD_DIR)
	
window: builddir
	rustc window.rs $(RS_FLAGS) --out-dir $(BUILD_DIR)