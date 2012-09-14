use std;
use glfw3;

import glfw3::*;

// Sorry, this file is a mess - trying to figure out the problem with the window

#[nolink]
#[cfg(target_os = "macos")]
#[link_args="-framework OpenGL -framework Cocoa"]
extern mod osx_frameworks {}

const GL_FALSE : int = 0;
const GL_TRUE  : int = 1;

fn main() {
    
    if (glfwInit() == 0) {
        fail(~"glfwInit() failed\n");
    }
    
    unsafe {
        glfw3::glfw3::glfwSetErrorCallback(errorCallback);
        glfw3::glfw3::glfwSetKeyCallback(keyCallback);
    }
    
    let mut window = glfwCreateWindow(800, 600, GLFW_WINDOWED, ~"Hello, I am a window.");
    
    io::println(fmt!("Window ptr: %d", window.ptr as int));
    
    if (ptr::is_null(window.ptr)) {
        glfwTerminate();
        io::println(~"Error: " + glfwErrorString(glfwGetError()));
        fail(~"glfwOpenWindow() failed\n");
    }
    
    // Test window size getter (works)
    let (width, height) = glfwGetWindowSize(&mut window);
    io::println(fmt!("Window size: %d x %d", width, height));
    
    // Test window size setter (works)
    glfwSetWindowSize(&mut window, 900, 500);
    let (width, height) = glfwGetWindowSize(&mut window);
    io::println(fmt!("Window size: %d x %d", width, height));
    
    // Test window position getter (works)
    let (xpos, ypos) = glfwGetWindowPos(&mut window);
    io::println(fmt!("Window position: (%d, %d)", xpos, ypos));
    
    // Test window position setter (works)
    glfwSetWindowPos(&mut window, 200, 100);
    let (xpos, ypos) = glfwGetWindowPos(&mut window);
    io::println(fmt!("Window position: (%d, %d)", xpos, ypos));
    
    // glfwSwapInterval(1); // uncomment to trigger error callback (no opengl context)
    
    glfwSetInputMode(&mut window, GLFW_STICKY_KEYS, GL_TRUE);
    
    let mut done = false;
    while (!done) {
        if (glfwGetKey(&mut window, GLFW_KEY_ESC) == GLFW_PRESS) {
            done = true;
        }
        
        // let (xpos, ypos) = glfwGetCursorPos(&mut window);
        // io::println(fmt!("Cursor position: (%d, %d)", xpos, ypos));
        
        // glfwSwapBuffers(&mut window);
        
        // io::println(~"Error: " + glfwErrorString(glfwGetError()));
    }  
    
    glfwTerminate();
}

extern fn errorCallback(key: libc::c_int, name: *libc::c_char) {
    unsafe { io::println(fmt!("Error in window %?: %s", key, str::unsafe::from_c_str(name))); }
}

extern fn keyCallback(window: GLFWwindow, key: libc::c_int, action: libc::c_int) {
    unsafe { io::println(fmt!("%?: %? %?", window, key, action)); }
}
