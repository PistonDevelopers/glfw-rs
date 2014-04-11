<!--
    Copyright 2013 The GLFW-RS Developers. For a full listing of the authors,
    refer to the AUTHORS file at the top-level directory of this distribution.

    Licensed under the Apache License, Version 2.0 (the "License");
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at

        http://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
    limitations under the License.
-->

# glfw-rs

GLFW bindings and wrapper for The Rust Programming Language.

## Example

~~~rust
extern crate native;
extern crate glfw;

use glfw::Context;

#[start]
fn start(argc: int, argv: **u8) -> int {
    // Run GLFW on the main thread
    native::start(argc, argv, main)
}

fn main() {
    let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    // Create a windowed mode window and its OpenGL context
    let window = glfw.create_window(300, 300, "Hello this is window", glfw::Windowed)
        .expect("Failed to create GLFW window.");

    // Make the window's context current
    window.make_context_current();

    // Loop until the user closes the window
    while !window.should_close() {
        // Swap front and back buffers
        window.swap_buffers();

        // Poll for and process events
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            println!("{}", event);
            match event {
                glfw::KeyEvent(glfw::KeyEscape, _, glfw::Press, _) => {
                    window.set_should_close(true)
                },
                _ => {},
            }
        }
    }
}
~~~

## Documentation

The [API docs](http://rust-ci.org/bjz/glfw-rs/doc/glfw/) are hosted on Rust CI.

## Prerequisites

To build glfw-rs you will need to [build glfw](http://www.glfw.org/docs/latest/compile.html) using the [BUILD_SHARED_LIBS](http://www.glfw.org/docs/latest/compile.html#compile_options) option.

Once you have built glfw-rs you might encouter the following error when running the examples:
'error while loading shared libraries: libglfw.so.3: ... '

Read the last part of [this article](http://www.brandonfoltz.com/2012/12/compile-glfw-on-ubuntu-and-fix-libglfw-so-cannot-open-error/) for information on how to fix this.


## Compilation

You can use [cargo-lite](https://github.com/cmr/cargo-lite):

~~~
cargo-lite.py install --git https://github.com/bjz/glfw-rs.git glfw-rs
~~~

Or use make manually to build the library and docs:

~~~
make
~~~

To build the examples:

~~~
make examples
~~~

Or a specific example:

~~~
make src/examples/window.rs
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
- [ozkriff/marauder](https://github.com/ozkriff/marauder/)

## Support

Contact `bjz` on irc.mozilla.org [#rust](http://mibbit.com/?server=irc.mozilla.org&channel=%23rust)
and [#rust-gamedev](http://mibbit.com/?server=irc.mozilla.org&channel=%23rust-gamedev),
or [post an issue](https://github.com/bjz/glfw-rs/issues/new) on Github.
