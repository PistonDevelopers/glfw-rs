use std;
use glfw;

import glfw::*;

fn main ()
{
	if (glfwInit() == 0)
	{
		fail("glfwInit() failed\n");
	}
	
	if (glfwOpenWindow(800, 600, 5, 6, 5, 0, 0, 0, GLFW_WINDOW) == 0)
	{
		fail("glfwOpenWindow() failed\n");
	}
	
	let major : @int = @0;
	let minor : @int = @0;
	let rev   : @int = @0;
	glfwGetGLVersion(major, minor, rev);
	
	let title = #fmt("Opengl version - %d.%d rev %d", *major, *minor, *rev);
	glfwSetWindowTitle(title);

    let mut done = false; 
    
	while (!done)
	{
		if (glfwGetKey(GLFW_KEY_ESC) == GLFW_PRESS  || !glfwGetWindowParam(GLFW_OPENED) as bool)
		{
            done = true;
		}

		glfwSwapBuffers();
    }  

	glfwTerminate();
}
