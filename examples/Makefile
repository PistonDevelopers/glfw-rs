RS_FLAGS	:= -g -L ../
BUILDDIR	:= build

all: modes time version window
	
builddir:
	@mkdir -p build
	
modes: builddir
	rustc modes.rs $(RS_FLAGS) --out-dir $(BUILDDIR)
	
time: builddir
	rustc time.rs $(RS_FLAGS) --out-dir $(BUILDDIR)
	
version: builddir
	rustc version.rs $(RS_FLAGS) --out-dir $(BUILDDIR)
	
window: builddir
	rustc window.rs $(RS_FLAGS) --out-dir $(BUILDDIR)