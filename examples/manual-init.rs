extern mod glfw;

fn main() {
    // Run this task on the main thread. Unlike C or C++, a Rust program
    // automatically starts a new thread, so this line is _essential_ to ensure
    // that the OS is able to update the window and recieve events from the user.
    do task::task().sched_mode(task::PlatformThread).spawn  {
        use core::private::finally::Finally;
        
        do (|| {
            glfw::set_error_callback(error_callback);
            
            if !glfw::init() { die!(~"Failed to initialize GLFW\n"); }
            
            let window =
                match glfw::Window::create(300, 300, "Hello this is window", glfw::Windowed) {
                    Some(w) => w,
                    None    => die!(~"Failed to open GLFW window")
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
            
        }).finally {
            glfw::terminate();    // terminate glfw on completion
        }
    }
}

fn error_callback(_error: libc::c_int, description: ~str) {
    io::println(fmt!("GLFW Error: %s", description));
}