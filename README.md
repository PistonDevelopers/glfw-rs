# glfw-rs

GLFW3 bindings for Rust. Based on alegalle's [rust_glfw](https://github.com/alegalle/rust_glfw) bindings but heavily modified to work with the latest development versions of [GLFW](https://github.com/elmindreda/glfw) and [Rust](https://github.com/mozilla/rust).

If you run into any problems, please let me know.

## Instructions

### Building glfw-rs

1. Make sure you have the latest version of GLFW built and installed on your system. This can be cloned from the project's [Github repository](https://github.com/elmindreda/glfw). Note:
  - The version that is available from [glfw.org](http://www.glfw.org/) is _very_ out of date, and will not work with these bindings.
  - GLFW builds as a static library as default which doesn't work well with Rust, so you'll have to set the `BUILD_SHARED_LIBS` CMake option to true _before_ you build and install it.
2. Make sure you have the latest version of Rust built and installed. As with GLFW, this can be clone from the [github repository](https://github.com/mozilla/rust).
3. Clone this repository: `$ git clone https://github.com/bjz/glfw-rs.git`, then `$ cd glfw-rs`.
4. Run `$ make`. This will build the library to the `./lib` directory.

### Building the examples

1. `$ cd glfw/examples`
2. `$ make` or for a specific example `$ make <example name>` (eg. `$ make window`)

### Using glfw-rs in your own projects

You can add the bindings to your `rustc` command like so: `$ rustc <filename> -L <path to glfw-rs directory>/lib`.

In order to use glfw-rs it is essential that you run your main loop from the main OS thread using `do task::task().sched_mode(task::PlatformThread).spawn { ... }`. Unlike C or C++, Rust programs automatically start on a separate thread from the main OS loop, so if you forget to do this the windows and input events won't be able to update. You can see an example of how to set this up in the [window example](https://github.com/bjz/glfw-rs/blob/master/examples/window.rs).

## Todo:
- Fix examples/time.rs
- Fix issues with callback functions:
  - glfwSetWindowRefreshCallback
  - glfwSetScrollCallback
- Work out how best to link to glfw on windows
- Register with Cargo Central.

~B☼
