extern mod glfw;

fn main() {
    glfw::set_error_callback(error_callback);
    
    do glfw::spawn {
        // Calling `Option::unwrap` will fail if `glfw::Window::create`
        // returns `None`. If you want to manually handle this eventuality
        // you can perform a match (see `examples/manual-init.rs`).
        let window = glfw::Window::create(300, 300, "Hello this is window", glfw::Windowed).unwrap();
        
        window.make_context_current();
        
        let mut done = false;
        
        while !done {
            glfw::poll_events();
            
            // Check if the window should close
            done = window.get_param(glfw::SHOULD_CLOSE) == glfw::TRUE
                || window.get_key(glfw::KEY_ESC)        == glfw::PRESS;
        }
        
        window.destroy();
    }
}

fn error_callback(_error: libc::c_int, description: ~str) {
    io::println(fmt!("GLFW Error: %s", description));
}