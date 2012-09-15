# glfw-rs

*Warning: these bindings are currently in development and may be incomplete, out of date or error-prone.*

GLFW3 bindings for Rust. Based on alegalle's [rust_glfw](https://github.com/alegalle/rust_glfw) bindings, but heavily modified to work with the latest development versions of [GLFW](https://github.com/elmindreda/glfw) and [Rust](https://github.com/mozilla/rust).

## Acknowledgements

- [alegalle](https://github.com/alegalle/) for providing a great starting point for these bindings.
- nmatsakis: for spending a ton of time helping me out on IRC
- brson (and nmatsakis): helping me figure out an essential threading problem

## Todo:
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
- Double check my pointer usage and code, get reviews and analysis from peers
- Add a tutorial to this readme
- Document wrapper functions
- Register with Cargo Central?
- Into the future:
  - Keep up to date with changes to GLFW3 and Rust
  - Create a separate windowing library (owindow-rs?) to abstract away from glfw

~B☼