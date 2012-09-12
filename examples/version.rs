use std;
use glfw3;

import glfw3::*;

fn main() {
    
    // get version string
    io::println(~"GLFW version: " + glfwGetVersionString());
    
    // get version tuple
    let (major, minor, rev) = glfwGetVersion();
    io::println(fmt!("GLFW version: %d.%d.%d", major, minor, rev));
    
}