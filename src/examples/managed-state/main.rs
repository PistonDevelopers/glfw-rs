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

#[feature(managed_boxes)];

extern mod glfw;

use std::local_data;

static tls_key: local_data::Key<@mut State> = &local_data::Key;

struct State {
    priv pos: (f64, f64),
}

impl State {
    pub fn update(x: f64, y: f64) {
        do local_data::get(tls_key) |opt| {
            match opt {
                Some(state) => state.pos = (x, y),
                None => local_data::set(tls_key, @mut State { pos: (x, y) }),
            }
        }
    }

    pub fn get_pos() -> Option<(f64, f64)> {
        do local_data::get(tls_key) |opt| {
            opt.map(|state| state.pos)
        }
    }
}

#[start]
fn start(argc: int, argv: **u8) -> int {
    std::rt::start_on_main_thread(argc, argv, main)
}

fn main() {
    do glfw::set_error_callback |_, msg| {
        println!("GLFW Error: {:s}", msg);
    }

    do glfw::start {
        let window = glfw::Window::create(300, 300, "Move cursor in window", glfw::Windowed)
            .expect("Failed to create GLFW window.");

        do window.set_cursor_pos_callback |_, x, y| {
            State::update(x, y);
        }

        do window.set_key_callback |win, key, _, action, _mods| {
            if action == glfw::Press && key == glfw::KeyEscape {
                win.set_should_close(true);
            }
        }
        window.make_context_current();

        while !window.should_close() {
            glfw::poll_events();

            do State::get_pos().map |(x, y)| {
                window.set_title(format!("({}, {})", x, y));
            };
        }
    }
}

