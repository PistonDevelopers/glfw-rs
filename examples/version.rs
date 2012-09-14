use std;
use glfw3;

import glfw3::*;

fn main() {
    
    // get version tuple
    let (major, minor, rev) = glfwGetVersion();
    io::println(fmt!("(%d, %d, %d)", major, minor, rev));
    
    // get version string
    io::println(~"GLFW version: " + glfwGetVersionString());
    
}