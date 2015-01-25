// Copyright 2014 The GLFW-RS Developers. For a full listing of the authors,
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

//! Demonstrates how concurrent rendering can be achieved
//! through the use of `RenderContext`s.

extern crate glfw;

use std::sync::mpsc::{channel, Receiver};
use glfw::{Action, Context, Key};
use std::thread::Builder;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw.create_window(300, 300, "Hello this is window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);

    let render_context = window.render_context();
    let (send, recv) = channel();

    let render_task = Builder::new().name("render task".to_string());
    let render_task_done = render_task.spawn(move || {
        render(render_context, recv);
    });

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }
    }

    // Tell the render task to exit.
    send.send(()).ok().expect("Failed signal to render thread.");

    // Wait for acknowledgement that the rendering was completed.
    let _ = render_task_done;
}

fn render(mut context: glfw::RenderContext, finish: Receiver<()>) {
    context.make_current();
    loop {
        // Check if the rendering should stop.
        if finish.try_recv() == Ok(()) { break };

        // Perform rendering calls

        context.swap_buffers();
    }

    // required on some platforms
    glfw::make_context_current(None);
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        }
        _ => {}
    }
}
