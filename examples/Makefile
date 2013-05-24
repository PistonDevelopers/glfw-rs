RS_FLAGS	:= -L ../lib
# RS_FLAGS	:= -g -L ../lib 		# causes "error: internal compiler error: create_function: unexpected sort of node"
BUILD_DIR	:= bin


all: callbacks clipboard defaults gl-struct modes title version window

builddir:
	@mkdir -p $(BUILD_DIR)

callbacks: builddir
	rustc callbacks.rs $(RS_FLAGS) --out-dir $(BUILD_DIR)

clipboard: builddir
	rustc clipboard.rs $(RS_FLAGS) --out-dir $(BUILD_DIR)

defaults: builddir
	rustc defaults.rs $(RS_FLAGS) --out-dir $(BUILD_DIR)

gl-struct: builddir
	rustc gl-struct.rs $(RS_FLAGS) --out-dir $(BUILD_DIR)

modes: builddir
	rustc modes.rs $(RS_FLAGS) --out-dir $(BUILD_DIR)

title: builddir
	rustc title.rs $(RS_FLAGS) --out-dir $(BUILD_DIR)

version: builddir
	rustc version.rs $(RS_FLAGS) --out-dir $(BUILD_DIR)

window: builddir
	rustc window.rs $(RS_FLAGS) --out-dir $(BUILD_DIR)

clean:
	rm -R -f $(BUILD_DIR)
