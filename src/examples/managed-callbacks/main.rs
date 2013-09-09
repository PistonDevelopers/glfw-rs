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

//! This example shows how managed objects can be accessed from callbacks

extern mod glfw;

use std::local_data;

static tls_key: local_data::Key<@mut TitleUpdater> = &local_data::Key;

struct TitleUpdater {
    window: @mut glfw::Window,
}

impl TitleUpdater {
    pub fn update(&self, title: &str) {
        self.window.set_title(title);
    }

    /* TLS management. */
    pub fn set(tu: @mut TitleUpdater) {
        local_data::set(tls_key, tu);
    }

    pub fn get() -> @mut TitleUpdater {
        do local_data::get(tls_key) |opt| {
            match opt {
                Some(x) => *x,
                None => fail!("Invalid TitleUpdater"),
            }
        }
    }
}

#[start]
fn start(argc: int, argv: **u8, crate_map: *u8) -> int {
    std::rt::start_on_main_thread(argc, argv, crate_map, main)
}

fn main() {
    do glfw::set_error_callback |_, msg| {
        printfln!("GLFW Error: %s", msg);
    }

    do glfw::start {
        let window = @mut glfw::Window::create(300, 300, "Move cursor in window", glfw::Windowed).unwrap();
        let title_updater = @mut TitleUpdater { window: window };
        TitleUpdater::set(title_updater); // Store in TLS.

        // Title updater must be in TLS and cannot be captured in the callback.
        do window.set_cursor_pos_callback |_, x, y| {
            TitleUpdater::get().update(fmt!("(%f %f)", x, y));
        }

        do window.set_key_callback |win, key, _, action, _mods| {
            if action == glfw::PRESS && key == glfw::KEY_ESCAPE {
                win.set_should_close(true);
            }
        }
        window.make_context_current();

        while !window.should_close() {
            glfw::poll_events();
        }
    }
}

