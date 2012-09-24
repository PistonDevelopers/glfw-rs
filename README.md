# glfw-rs

*Warning: these bindings are currently in development and may be incomplete, out of date or error-prone.*

GLFW3 bindings for Rust. Based on alegalle's [rust_glfw](https://github.com/alegalle/rust_glfw) bindings but heavily modified to work with the latest development versions of [GLFW](https://github.com/elmindreda/glfw) and [Rust](https://github.com/mozilla/rust).

## What is GLFW?

From [glfw.org](http://www.glfw.org/):

> GLFW is a free, Open Source, multi-platform library for opening a window, creating an OpenGL context and managing input. It is easy to integrate into existing applications and does not lay claim to the main loop.

> GLFW is written in C and has native support for Windows, Mac OS X and many Unix-like systems using the X Window System, such as Linux and FreeBSD.

Basically GLFW is a lightweight alternative to [SDL](http://www.libsdl.org/) and [SFML](http://www.sfml-dev.org/).

## Instructions

### Building glfw-rs

1. Make sure you have the latest version of GLFW built and installed on your system. This can be cloned from the project's [Github repository](https://github.com/elmindreda/glfw). Note: the version that is available from [glfw.org](http://www.glfw.org/) is _very_ out of date, and will not work with these bindings.
2. Make sure you have the latest version of Rust built and installed. As with GLFW, this can be clone from the [github repository](https://github.com/mozilla/rust).
3. Clone this repository: `$ git clone https://github.com/bjz/glfw3-rs.git`, then `$ cd glfw-rs`.
4. Run `$ make`. This will build the library to the `./lib` directory.

### Building the examples

1. `$ cd glfw/examples`
2. `$ make` or for a specific example `$ make <example name>` (eg. `$ make window`)
3. run an example `$ ./build/<example name>` (eg. `$ ./build/window`)

### Using glfw-rs in your own projects

You can add the bindings to your `rustc` command like so: `$ rustc <filename> -L <path to glfw-rs directory>/lib`.

In order to use glfw-rs it is essential that you run your main loop from the main OS thread using `do task::task().sched_mode(task::PlatformThread).spawn { ... }`. Unlike C or C++, Rust programs automatically start on a separate thread from the main OS loop, so if you forget to do this bad things will happen! You can see an example of how to set this up in the [window example file](https://github.com/bjz/glfw3-rs/blob/master/examples/window.rs).

## Acknowledgements

- [Alegalle](https://github.com/alegalle/): for providing an excellent starting point for these bindings.
- [Niko Matsakis](https://github.com/nikomatsakis), [Brian Anderson](https://github.com/brson/) and [Patrick Walton](https://github.com/pcwalton): for generously spending a ton of their time helping me out with my many problems and questions. Cheers guys!

## Todo:
- Fix examples/time.rs
- Wrap callback functions so that the user doesn't need to deal with external functions:
  - glfwSetErrorCallback
  - glfwSetWindowSizeCallback
  - glfwSetWindowCloseCallback
  - glfwSetWindowRefreshCallback
  - glfwSetWindowFocusCallback
  - glfwSetWindowIconifyCallback
  - glfwSetKeyCallback
  - glfwSetCharCallback
  - glfwSetMouseButtonCallback
  - glfwSetCursorPosCallback
  - glfwSetCursorEnterCallback
  - glfwSetScrollCallback
- Implement and wrap the last outstanding function bindings:
  - glfwSetWindowUserPointer
  - glfwGetWindowUserPointer
- Document wrapper functions
- Register with Cargo Central?
- Into the future:
  - Keep up to date with the constant changes to GLFW3 and Rust
  - Create a separate windowing library (owindow-rs?) to abstract away from glfw

~B☼