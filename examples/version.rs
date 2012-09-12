use std;
use glfw3;

import glfw3::*;

fn main() {
    
    // get version string
    io::println(~"GLFW version: " + glfwGetVersionString());
    
    // get version tuple
    let version = glfwGetVersion();
    match version {
        (major, minor, rev) => {
            io::println(fmt!("GLFW version: %d.%d.%d", major, minor, rev));
        }
    }
    
}