extern mod GLFW (name = "glfw");
use glfw = GLFW::hl;

fn main() {
    // get version tuple
    let version = glfw::get_version();
    io::println(fmt!("(%d, %d, %d)", version.major, version.minor, version.rev));
    
    // get version string
    io::println(~"GLFW version: " + glfw::get_version_string());
}