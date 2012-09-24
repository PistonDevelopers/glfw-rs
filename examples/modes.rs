extern mod std;
extern mod glfw3;
use glfw3::*;

fn main() {
    if (glfwInit() == 0) {
        fail(~"glfwInit() failed\n");
    }
    
    let dt_mode = glfwGetDesktopMode();
    io::println(~"Desktop mode:\n" + mode_str(dt_mode));
    
    let modes = glfwGetVideoModes();
    
    io::println(~"Available modes:");
    modes.map(|m| { io::println(mode_str(m)) });
    
    glfwTerminate();
}

fn mode_str(mode: GLFWvidmode) -> ~str {
    fmt!("%d x %d\t%d (%d %d %d)",
         mode.width as int, mode.height as int,
         (mode.redBits + mode.blueBits + mode.greenBits) as int,
         mode.redBits as int, mode.blueBits as int, mode.greenBits as int)
}