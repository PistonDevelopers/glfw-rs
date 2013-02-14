extern mod glfw3;

fn main() {
    glfw3::set_error_callback(error_callback);
    
    do glfw3::spawn {
        let window =
            match glfw3::Window::create(400, 400, "English 日本語 русский язык 官話", glfw3::Windowed) {
                Some(w) => { w }
                None => {
                    glfw3::terminate();
                    die!(~"Failed to open GLFW window");
                }
            };
        
        window.make_context_current();
        glfw3::set_swap_interval(1);
        
        while window.get_param(glfw3::SHOULD_CLOSE) == 0 {
            glfw3::wait_events();
        }
        
        window.destroy();
    }
}

fn error_callback(_error: libc::c_int, description: ~str) {
    io::println(fmt!("GLFW Error: %s", description));
}