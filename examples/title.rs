extern mod glfw;

fn main() {
    glfw::set_error_callback(error_callback);

    do glfw::spawn {
        let window = glfw::Window::create(400, 400, "English 日本語 русский язык 官話", glfw::Windowed).unwrap();

        window.make_context_current();
        window.set_key_callback(key_callback);
        glfw::set_swap_interval(1);

        while !window.should_close() {
            glfw::wait_events();
        }
    }
}

fn key_callback(window: &glfw::Window, key: libc::c_int, action: libc::c_int, _: libc::c_int) {
    if action == glfw::PRESS && key == glfw::KEY_ESCAPE {
        window.set_should_close(true);
    }
}

fn error_callback(_: libc::c_int, description: ~str) {
    io::println(fmt!("GLFW Error: %s", description));
}
