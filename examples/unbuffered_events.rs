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

use glfw::{Action, Context, Key, WindowEvent, RenderContext};

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw.create_window(300, 300, "Hello this is window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_all_polling(true);
    let mut render_context = window.render_context();

    // Render asynchronously from the main event loop. This prevents a pause in rendering
    // during some window interactions (e.g. resizing) on platforms like Windows.
    let thread = std::thread::spawn(move || {
        render_context.make_current();
        while !render_context.should_close() {
            for (_, event) in glfw::flush_messages(&events) {
                handle_window_event(&mut render_context, event);
            }
            render_context.swap_buffers();
        }
    });

    while !window.should_close() {
        glfw.wait_events_unbuffered(|window_id, event| {
            // Multiple windows may be identified by their `window_id`
            assert_eq!(window.window_id(), window_id);

            // Intercept the close request and reset the flag
            if let (_, WindowEvent::Close) = event {
                window.set_should_close(false);
            };

            // Forward the event to the render thread via the `events` receiver
            // to be processed asynchronously from the main event loop thread.
            // Returning `None` here would consume the event.
            Some(event)
        });
    }

    thread.join().unwrap();
}

fn handle_window_event(render_context: &mut RenderContext, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Close | glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            render_context.set_should_close(true);
            render_context.post_empty_event();
        }
        _ => {}
    }
}
