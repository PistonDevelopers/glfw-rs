use std;
use glfw;

import glfw::*;

fn main ()
{
	unsafe {
		
	if (glfwInit() == 0)
	{
		fail("glfwInit() failed\n");
	}
	
	if (glfwOpenWindow(800, 600, 5, 6, 5, 0, 0, 0, GLFW_WINDOW) == 0)
	{
		fail("glfwOpenWindow() failed\n");
	}
	
	let major : int = 0;
	let minor : int = 0;
	let rev : int = 0;
	glfwGetGLVersion(ptr::addr_of(major), ptr::addr_of(minor), ptr::addr_of(rev));
	
	let title = #fmt("Opengl version - %d.%d rev %d", major, minor, rev);
	glfwSetWindowTitle(title);

	while (true)
	{
		if (glfwGetKey(GLFW_KEY_ESC) == GLFW_PRESS  || !glfwGetWindowParam(GLFW_OPENED) as bool)
		{
            break;
		}

		glfwSwapBuffers();
    }  

	glfwTerminate();
	
	}  //unsafe
}
