extern mod glfw3;

fn main() {
    glfw3::set_error_callback(error_callback);
    
    do glfw3::spawn {
        
        glfw3::window_hint(glfw3::VISIBLE, 0);
        
        let window =
            match glfw3::Window::create(640, 480, "Defaults", glfw3::Windowed) {
                Some(w) => { w }
                None => {
                    glfw3::terminate();
                    die!(~"Failed to open GLFW window");
                }
            };
        
        window.make_context_current();
        
        let (width, height) = window.get_size();
        io::println(fmt!("window size: %? x %?", width, height));
        
        io::println(fmt!("Context version major: %?",     window.get_param(glfw3::CONTEXT_VERSION_MAJOR)));
        io::println(fmt!("Context version minor: %?",     window.get_param(glfw3::CONTEXT_VERSION_MINOR)));
        io::println(fmt!("OpenGL forward compatible: %?", window.get_param(glfw3::OPENGL_FORWARD_COMPAT)));
        io::println(fmt!("OpenGL debug context: %?",      window.get_param(glfw3::OPENGL_DEBUG_CONTEXT)));
        io::println(fmt!("OpenGL profile: %?",            window.get_param(glfw3::OPENGL_PROFILE)));
        
        // TODO: Test OpenGL defaults: 
        //   - GL_RED_BITS
        //   - GL_GREEN_BITS
        //   - GL_BLUE_BITS
        //   - GL_ALPHA_BITS
        //   - GL_DEPTH_BITS
        //   - GL_STENCIL_BITS
        //   - GL_STEREO
        //   - GL_SAMPLES_ARB
        
        window.destroy();
    }
}

fn error_callback(_error: libc::c_int, description: ~str) {
    io::println(fmt!("GLFW Error: %s", description));
}