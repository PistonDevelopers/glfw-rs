extern mod glfw3;

fn main() {
    
    // get version tuple
    let version = glfw3::get_version();
    io::println(fmt!("(%d, %d, %d)", version.major, version.minor, version.rev));
    
    // get version string
    io::println(~"GLFW version: " + glfw3::get_version_string());
    
}