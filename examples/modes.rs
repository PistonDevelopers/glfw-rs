use std;
use glfw3;
import to_str::ToStr;
import glfw3::*;

fn main() {
    
    let dt_mode = glfwGetDesktopMode();
    io::println(mode_str(dt_mode));
    
    let modes = glfwGetVideoModes();
    
    for vec::each(modes) |m| {
        io::println(mode_str(m));
    }
    
}

fn mode_str(mode: GLFWvidmode) -> ~str {
    fmt!("%dpx x %dpx r: %d g: %d b: %d", mode.width, mode.height , mode.redBits, mode.blueBits, mode.greenBits)
}