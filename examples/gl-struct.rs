// Copyright 2013 The GLFW-RS Developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! This example illustrates how a OpenGL function struct might be
//! be used for rendering a viewport resize from inside a callback.

extern mod glfw;

use std::libc;
use std::rt;

type GLint = i32;

// This would be supplied be an external library
struct GL {
    // The fields would actually contain pointers to extern functions
    Viewport: @fn(x: GLint, y: GLint, width: GLint, height: GLint)
}

impl GL {
    fn init() -> GL {
        GL {
            Viewport: |x, y, width, height| {
                // Obviously this doesn't actually call glViewport
                println(fmt!("glResize(%?, %?, %?, %?)", x, y, width, height));
            }
        }
    }

    #[inline(always)]
    fn Viewport(&self, x: GLint, y: GLint, width: GLint, height: GLint) {
        (self.Viewport)(x, y, width, height)
    }
}

#[start]
fn main(argc: int, argv: **u8, crate_map: *u8) -> int {
    do rt::start_on_main_thread(argc, argv, crate_map) {
        glfw::set_error_callback(error_callback);

        do glfw::start {
            let window = glfw::Window::create(640, 480, "Resize the window to call glViewport", glfw::Windowed).unwrap();

            window.make_context_current();

            let gl = GL::init();

            do window.set_size_callback |_, width, height| {
                // Rust doesn't have global state, so we use a borrowed pointer
                // to the gl struct in order to perform rendering operations
                // in other functions
                render_resize(&gl, width, height);
            }

            while !window.should_close() {
                glfw::poll_events();
            }
        }
    }
}

fn render_resize(gl: &GL, width: int, height: int) {
    gl.Viewport(0, 0, width as GLint, height as GLint);
}

fn error_callback(_: libc::c_int, description: ~str) {
    println(fmt!("GLFW Error: %s", description));
}
