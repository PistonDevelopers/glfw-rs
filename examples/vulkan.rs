// Copyright 2016 The GLFW-RS Developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[cfg(not(feature = "vulkan"))]
fn main() {
    eprintln!("run with: --features vulkan")
}

#[cfg(feature = "vulkan")]
use ash::vk;

#[cfg(feature = "vulkan")]
use std::ptr;

#[cfg(feature = "vulkan")]
fn main() {
    let mut glfw = glfw::init_no_callbacks().unwrap();

    glfw.window_hint(glfw::WindowHint::Visible(true));
    glfw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));

    let (window, _) = glfw
        .create_window(640, 480, "Defaults", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    assert!(glfw.vulkan_supported());

    let required_extensions = glfw.get_required_instance_extensions().unwrap_or(vec![]);

    //VK_KHR_surface will always be available if the previous operations were successful
    assert!(required_extensions.contains(&"VK_KHR_surface".to_string()));

    println!("Vulkan required extensions: {:?}", required_extensions);

    //Load up all the entry points using 0 as the VkInstance,
    //since you can't have an instance before you get vkCreateInstance...
    let entry = unsafe { ash::Entry::load().expect("Failed to load Vulkan library.") };

    let instance: ash::Instance = unsafe { create_instance(&entry, &required_extensions) };

    let mut surface: std::mem::MaybeUninit<vk::SurfaceKHR> = std::mem::MaybeUninit::uninit();

    if window.create_window_surface(instance.handle(), ptr::null(), surface.as_mut_ptr())
        != vk::Result::SUCCESS
    {
        panic!("Failed to create GLFW window surface.");
    }

    // Use other vulkan stuff here.

    println!("Vulkan instance successfully created. Destruction is automatic with Drop.");
}

#[cfg(feature = "vulkan")]
unsafe fn create_instance(entry: &ash::Entry, extensions: &Vec<String>) -> ash::Instance {
    // Turn the list of extensions into a format that can be passed in InstanceCreateInfo
    let extensions: Vec<std::ffi::CString> = extensions
        .iter()
        .map(|ext| std::ffi::CString::new(ext.clone()).expect("Failed to convert extension name"))
        .collect();
    let extension_pointers: Vec<*const i8> = extensions.iter().map(|ext| ext.as_ptr()).collect();
    //This is literally the bare minimum required to create a blank instance
    //You'll want to fill in this with real data yourself
    let info: vk::InstanceCreateInfo =
        vk::InstanceCreateInfo::default().enabled_extension_names(&extension_pointers);

    unsafe {
        entry
            .create_instance(&info, None)
            .expect("Unable to create instance.")
    }
}
