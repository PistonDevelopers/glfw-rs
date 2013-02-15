extern mod glfw;

fn main() {
    glfw::set_error_callback(error_callback);
    
    do glfw::spawn {
        
        glfw::window_hint(glfw::VISIBLE, glfw::TRUE);
        
        let window =
            match glfw::Window::create(640, 480, "Defaults", glfw::Windowed) {
                Some(w) => { w }
                None => {
                    glfw::terminate();
                    die!(~"Failed to open GLFW window");
                }
            };
        
        window.make_context_current();
        
        let (width, height) = window.get_size();
        io::println(fmt!("window size: %? x %?", width, height));
        
        io::println(fmt!("Context version major: %?",     window.get_param(glfw::CONTEXT_VERSION_MAJOR)));
        io::println(fmt!("Context version minor: %?",     window.get_param(glfw::CONTEXT_VERSION_MINOR)));
        io::println(fmt!("OpenGL forward compatible: %?", window.get_param(glfw::OPENGL_FORWARD_COMPAT)));
        io::println(fmt!("OpenGL debug context: %?",      window.get_param(glfw::OPENGL_DEBUG_CONTEXT)));
        io::println(fmt!("OpenGL profile: %?",            window.get_param(glfw::OPENGL_PROFILE)));
        
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