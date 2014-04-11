<!--
    Copyright 2013-2014 The GLFW-RS Developers. For a full listing of the authors,
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

GLFW wrapper for The Rust Programming Language.

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

## Compilation

The makefile comes with a number of targets:

- `make`: library, examples, docs
- `make lib`: libs only
- `make doc`: docs only
- `make check`: build and run the tests
- `make examples`: build all examples
- `make src/examples/<example>.rs`: build a specific example
- `make clean`: remove all build files

You can also use [cargo-lite](https://github.com/cmr/cargo-lite):

~~~
cargo-lite.py install --git https://github.com/bjz/glfw-rs.git glfw-rs
~~~

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
