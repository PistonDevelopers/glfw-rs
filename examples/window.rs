extern mod glfw;

fn main() {
    glfw::set_error_callback(error_callback);
    
    do glfw::spawn {
        let window =
            match glfw::Window::create(300, 300, "Hello this is window", glfw::Windowed) {
                Some(w) => w,
                None => die!(~"Failed to open GLFW window")
            };
        
        window.make_context_current();
        
        let mut done = false;
        
        while !done {
            glfw::poll_events();
            if (window.get_key(glfw::KEY_ESC) == glfw::PRESS || window.get_param(glfw::SHOULD_CLOSE) != 0) {
                done = true;
            }
        }
        
        window.destroy();
    }
}

fn error_callback(_error: libc::c_int, description: ~str) {
    io::println(fmt!("GLFW Error: %s", description));
}