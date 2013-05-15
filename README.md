# glfw-rs

GLFW3 bindings and wrapper for The Rust Programming Language.

## Instructions

### Building glfw-rs

1. Make sure you have the latest version of the GLFW 3 development version (not 2.7, which is on most package managers) built and installed on your system. This can be cloned from the project's [Github repository](https://github.com/glfw/glfw). GLFW builds as a static library by default which doesn't work well with Rust, so you'll have to set the `BUILD_SHARED_LIBS` CMake option to true _before_ you build and install it.
2. Make sure you have the latest [version of Rust](https://github.com/mozilla/rust) built and installed from the `incoming` branch.
3. Clone this repository: `$ git clone https://github.com/bjz/glfw-rs.git`, then `$ cd glfw-rs`.
4. Run `$ make`. This will build the library to the `./lib` directory.

### Building the examples

1. `$ cd glfw/examples`
2. `$ make` or for a specific example `$ make <example name>` (eg. `$ make window`)

## glfw-rs in use

- [Jeaye/q3](https://github.com/Jeaye/q3)
- [cyndis/rsmc](https://github.com/cyndis/rsmc/)
- [bjz/open.gl-tutorials](https://github.com/bjz/open.gl-tutorials)
