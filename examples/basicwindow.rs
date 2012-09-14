use std;
use glfw3;

import glfw3::*;

fn main() {
    
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
        if (glfwGetKey(&mut window, GLFW_KEY_ESC) == GLFW_PRESS || glfwGetWindowParam(&mut window, GLFW_CLOSE_REQUESTED) != 0) {
            done = true;
        }
    }  
    
    glfwTerminate();
}