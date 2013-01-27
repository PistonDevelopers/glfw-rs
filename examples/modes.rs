extern mod glfw3;

fn main() {
    if (glfw3::init() == false) {
        fail(~"glfwInit() failed\n");
    }
    
    io::println(fmt!("Primary monitor: %s\n", glfw3::get_primary_monitor().get_name()));
    
    io::println(~"Available monitors\n\
                  ------------------");
    
    glfw3::get_monitors().map(|monitor| {
        io::println(fmt!("%s:", monitor.get_name()));
        
        do monitor.get_video_modes().map |mode| {
            io::println(fmt!("  %s", mode.to_str()));
        }
    });
    
    glfw3::terminate();
}