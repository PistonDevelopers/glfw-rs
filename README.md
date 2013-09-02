# glfw-rs

GLFW bindings and wrapper for The Rust Programming Language.

## Example code

~~~rust
extern mod glfw;

#[start]
fn start(argc: int, argv: **u8, crate_map: *u8) -> int {
    // Run GLFW on the main thread
    std::rt::start_on_main_thread(argc, argv, crate_map, main)
}

fn main() {
    // Set an error callback
    do glfw::set_error_callback |_, description| {
        printfln!("GLFW Error: %s", description);
    }

    // Initialize the library
    do glfw::start {
        // Create a windowed mode window and its OpenGL context
        let window = glfw::Window::create(300, 300, "Hello this is window", glfw::Windowed).unwrap();

        // Make the window's context current
        window.make_context_current();

        // Loop until the user closes the window
        while !window.should_close() {
            // Swap front and back buffers
            window.swap_buffers();

            // Poll for and process events
            window.poll_events();   // This will not be necessary in future versions
            glfw::poll_events();
        }
    }
}
~~~

## Compilation

Building the library
~~~
rustpkg build glfw
~~~

Building the examples
~~~
rustpkg build examples
~~~

Building a specific example
~~~
rustpkg build examples/callbacks
~~~

### Note

glfw-rs wraps [glfw v3](http://www.glfw.org/). If you encounter lots of errors
like: `undefined reference to 'glfwSetScrollCallback'`, make sure you version
of glfw is up to date.

## glfw-rs in use

- [sebcrozet/kiss3d](https://github.com/sebcrozet/kiss3d)
- [Jeaye/q3](https://github.com/Jeaye/q3)
- [cyndis/rsmc](https://github.com/cyndis/rsmc/)
- [mozilla/servo](https://github.com/mozilla/servo)
