extern mod std;
extern mod glfw3;
use glfw3::*;

fn main() {
    if (glfwInit() == 0) {
        fail(~"glfwInit() failed\n");
    }
    
    let dt_mode = glfwGetDesktopMode();
    io::println(~"Desktop mode:\n" + mode_str(&dt_mode));
    
    let modes = glfwGetVideoModes();
    
    io::println(~"Available modes:");
    modes.map(|m| io::println(mode_str(m)));
    
    glfwTerminate();
}

fn mode_str(mode: &GLFWvidmode) -> ~str {
    fmt!("%? x %?\t%? (%? %? %?)",
         mode.width, mode.height,
         (mode.redBits + mode.blueBits + mode.greenBits),
         mode.redBits, mode.blueBits, mode.greenBits)
}