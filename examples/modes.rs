extern mod GLFW (name = "glfw");
use glfw = GLFW::hl;

fn main() {
    do glfw::spawn {
        io::println(fmt!("Primary monitor: %s\n", glfw::get_primary_monitor().get_name()));
        
        io::println(~"Available monitors\n\
                      ------------------");
        
        glfw::get_monitors().map(|monitor| {
            io::println(fmt!("%s:", monitor.get_name()));
            
            do monitor.get_video_modes().map |mode| {
                io::println(fmt!("  %s", mode.to_str()));
            }
        });
    }
}