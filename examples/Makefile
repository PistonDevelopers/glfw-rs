RS_FLAGS	:= -g -L ../lib
BUILD_DIR	:= build

all: modes time version window
	
builddir:
	@mkdir -p $(BUILD_DIR)
	
modes: builddir
	rustc modes.rs $(RS_FLAGS) --out-dir $(BUILD_DIR)
	
time: builddir
	rustc time.rs $(RS_FLAGS) --out-dir $(BUILD_DIR)
	
version: builddir
	rustc version.rs $(RS_FLAGS) --out-dir $(BUILD_DIR)
	
window: builddir
	rustc window.rs $(RS_FLAGS) --out-dir $(BUILD_DIR)