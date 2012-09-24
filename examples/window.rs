extern mod std;
extern mod glfw3;

use glfw3::*;

fn main() {
    
    // Run this task on the main thread. Unlike C or C++, a Rust program
    // automatically starts a new thread, so this line is _essential_ to ensure
    // that the OS is able to update the window and recieve events from the user.
    do task::task().sched_mode(task::PlatformThread).spawn {
        if (glfwInit() == 0) {
            glfwTerminate();
            fail(~"glfwInit() failed\n");
        }
        
        let mut window = glfwCreateWindow(300, 300, GLFW_WINDOWED, ~"Hello, I am a window.");
        
        io::println(fmt!("Window ptr: %d", window.ptr as int));
        
        if (ptr::is_null(window.ptr)) {
            glfwTerminate();
            io::println(~"Error: " + glfwErrorString(glfwGetError()));
            fail(~"glfwOpenWindow() failed\n");
        }
        
        let mut done = false;
        
        while (!done) {
            glfwPollEvents();
            if (glfwGetKey(&window, GLFW_KEY_ESC) == GLFW_PRESS || glfwGetWindowParam(&window, GLFW_CLOSE_REQUESTED) != 0) {
                done = true;
            }
        }
        
        glfwTerminate();
    }
}