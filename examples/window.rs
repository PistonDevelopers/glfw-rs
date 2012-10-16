extern mod std;
extern mod glfw3;

fn main() {
    
    // Run this task on the main thread. Unlike C or C++, a Rust program
    // automatically starts a new thread, so this line is _essential_ to ensure
    // that the OS is able to update the window and recieve events from the user.
    do task::task().sched_mode(task::PlatformThread).spawn {
        if (glfw3::init() == 0) {
            glfw3::terminate();
            fail(~"glfwInit() failed\n");
        }
        
        let mut window = glfw3::create_window(300, 300, glfw3::WINDOWED, ~"Hello, I am a window.");
        
        io::println(fmt!("Window ptr: %d", window.ptr as int));
        
        if (ptr::is_null(window.ptr)) {
            glfw3::terminate();
            io::println(~"Error: " + glfw3::error_string(glfw3::get_error()));
            fail(~"glfwOpenWindow() failed\n");
        }
        
        window.make_context_current();
        
        let mut done = false;
        
        while !done {
            glfw3::poll_events();
            if (window.get_key(glfw3::KEY_ESC) == glfw3::PRESS || window.get_param(glfw3::CLOSE_REQUESTED) != 0) {
                done = true;
            }
        }
        
        glfw3::terminate();
    }
}