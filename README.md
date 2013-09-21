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

## FAQ

_I get lots of errors like: `undefined reference to 'glfwSetScrollCallback'`_

glfw-rs wraps [glfw 3.0](http://www.glfw.org/). Version 2.7 was out for a
_long_ time, and may still be hanging around on package managers. If you
encounter these kinds of errors, make sure you version of glfw is up to date.

_Ok, so I have windowing sorted, now where do I find OpenGL?_

You can use the function pointer loader, [gl-rs](https://github.com/bjz/gl-rs),
or the [OpenGL-ES bindings](https://github.com/mozilla-servo/rust-opengles).

## glfw-rs in use

- [sebcrozet/kiss3d](https://github.com/sebcrozet/kiss3d)
- [Jeaye/q3](https://github.com/Jeaye/q3)
- [cyndis/rsmc](https://github.com/cyndis/rsmc/)
- [mozilla/servo](https://github.com/mozilla/servo)

## Support

Contact `bjz` on irc.mozilla.org [#rust](http://mibbit.com/?server=irc.mozilla.org&channel=%23rust)
and [#rust-gamedev](http://mibbit.com/?server=irc.mozilla.org&channel=%23rust-gamedev),
or [post an issue](https://github.com/bjz/glfw-rs/issues/new) on Github.
