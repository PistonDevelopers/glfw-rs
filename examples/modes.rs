extern mod std;
extern mod glfw3;

fn main() {
    if (glfw3::init() == 0) {
        fail(~"glfwInit() failed\n");
    }
    
    let dt_mode = glfw3::get_desktop_mode();
    io::println(~"Desktop mode:\n" + mode_str(&dt_mode));
    
    let modes = glfw3::get_video_modes();
    
    io::println(~"Available modes:");
    modes.map(|m| io::println(mode_str(m)));
    
    glfw3::terminate();
}

fn mode_str(mode: &glfw3::VidMode) -> ~str {
    fmt!("%? x %?\t%? (%? %? %?)",
         mode.width, mode.height,
         (mode.redBits + mode.blueBits + mode.greenBits),
         mode.redBits, mode.blueBits, mode.greenBits)
}