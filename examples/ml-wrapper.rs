/**
 * This example shows how you can use the mid-level wrapper directly as opposed to the
 * high-level wrapper. You may want to do this if you prefer a near to 1:1 mapping
 * to the GLFW API (the high level bindings alter it considerably).
 */

extern mod GLFW (name = "glfw");
use glfw = GLFW::ml;    // use mid-level bindings

fn main() {
    // Run this task on the main thread. Unlike C or C++, a Rust program
    // automatically starts a new thread, so this line is _essential_ to ensure
    // that the OS is able to update the window and recieve events from the user.
    do task::spawn_sched(task::PlatformThread) {
        use core::unstable::finally::Finally;
        
        // The `glfw::{TRUE, FALSE}` constants are added for convenience. You could also use
        // the `GL_TRUE` or `GL_FALSE` constants from you OpenGL bindings.
        if glfw::init() == glfw::FALSE {
            fail!(~"Failed to initialize GLFW");
        }
        
        // Using `do (|| { ... }).finally { glfw::terminate() }` allows us to ensure that
        // `glfw::terminate` is called even when failure occurs during runtime
        do (|| {
            glfw::set_error_callback(error_callback);
            
            let window = glfw::create_window(300, 300, "Hello this is window", ptr::null(), ptr::null());
            
            if window.is_null() { fail!(~"Failed to initialize GLFW window\n"); }
            
            glfw::set_key_callback(window, key_callback);
            glfw::make_context_current(window);
            
            while glfw::window_should_close(window) == glfw::FALSE {
                glfw::poll_events();
            }
            
        }).finally {
            glfw::terminate();    // terminate glfw on completion
        }
    }
}

extern fn key_callback(window: *glfw::GLFWwindow, key: libc::c_int, action: libc::c_int) {
    if action == glfw::PRESS && key == glfw::KEY_ESCAPE {
        glfw::set_window_should_close(window, glfw::TRUE);
    }
}

extern fn error_callback(_error: libc::c_int, description: *libc::c_char) {
    io::println(fmt!("GLFW Error: %s", unsafe { str::raw::from_c_str(description) }));
}
