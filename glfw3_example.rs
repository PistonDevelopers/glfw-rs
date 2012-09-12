use std;
use glfw3;

import glfw3::*;

fn main () {
    
    if (glfwInit() == 0) {
        fail(~"glfwInit() failed\n");
    }
    
    let mut window = glfwCreateWindow(800, 600, GLFW_WINDOWED, ~"Hello, I am a window.");
    if (window.ptr as bool == false) {
        fail(~"glfwOpenWindow() failed\n");
    }

    let mut done = false; 
    
    while (!done) {
        if (glfwGetKey(&mut window, GLFW_KEY_ESC) == GLFW_PRESS || !glfwGetWindowParam(&mut window, GLFW_CLOSE_REQUESTED) as bool) {
            done = true;
        }

        glfwSwapBuffers(&mut window);
    }  

    glfwTerminate();
}
