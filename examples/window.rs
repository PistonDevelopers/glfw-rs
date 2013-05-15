extern mod glfw;

fn main() {
    glfw::set_error_callback(error_callback);

    do glfw::spawn {
        // Calling `Option::unwrap` will fail if `glfw::Window::create`
        // returns `None`. If you want to manually handle this eventuality
        // you can perform a match (see `examples/manual-init.rs`).
        let window = glfw::Window::create(300, 300, "Hello this is window", glfw::Windowed).get();

        window.set_key_callback(key_callback);
        window.make_context_current();

        while !window.should_close() {
            glfw::poll_events();
        }
    }
}

fn key_callback(window: &glfw::Window, key: libc::c_int, action: libc::c_int) {
    if action == glfw::PRESS && key == glfw::KEY_ESCAPE {
        window.set_should_close(true);
    }
}

fn error_callback(_error: libc::c_int, description: ~str) {
    io::println(fmt!("GLFW Error: %s", description));
}