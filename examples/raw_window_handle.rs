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

extern crate glfw;
#[cfg(feature = "raw-window-handle-v0-6")]
extern crate raw_window_handle_0_6 as raw_window_handle;

#[cfg(not(feature = "raw-window-handle-v0-6"))]
extern crate raw_window_handle_0_5 as raw_window_handle;

use glfw::{Action, Context, Key};
#[cfg(not(feature = "raw-window-handle-v0-6"))]
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
#[cfg(feature = "raw-window-handle-v0-6")]
use raw_window_handle::{HasWindowHandle, RawWindowHandle};

fn main() {
    let mut glfw = glfw::init_no_callbacks().unwrap();

    let (mut window, events) = glfw
        .create_window(300, 300, "Hello this is window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();
    #[cfg(feature = "raw-window-handle-v0-6")]
    let raw = window.window_handle().unwrap().as_raw();
    #[cfg(not(feature = "raw-window-handle-v0-6"))]
    let raw = window.raw_window_handle();

    match raw {
        RawWindowHandle::Win32(handle) => println!("raw handle: {:?}", handle),
        RawWindowHandle::Xlib(handle) => println!("raw handle: {:?}", handle),
        RawWindowHandle::Wayland(handle) => println!("raw handle: {:?}", handle),
        RawWindowHandle::AppKit(handle) => println!("raw handle: {:?}", handle),
        _ => unimplemented!(),
    }

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        _ => {}
    }
}
