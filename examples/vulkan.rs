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
#![cfg(feature = "vulkan")]

extern crate glfw;
extern crate vk_sys;

use std::os::raw::c_void;
use std::mem;
use std::ptr;

use vk_sys::{
    self as vk,
    EntryPoints,
    InstancePointers,
    InstanceCreateInfo,
    Instance as VkInstance,
    Result as VkResult
};

use glfw::Context;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(glfw::WindowHint::Visible(true));

    let (mut window, _) = glfw.create_window(640, 480, "Defaults", glfw::WindowMode::Windowed)
                              .expect("Failed to create GLFW window.");

    window.make_current();

    assert!(glfw.vulkan_supported());

    let required_extensions = glfw.get_required_instance_extensions().unwrap_or(vec![]);

    //VK_KHR_surface will always be available if the previous operations were successful
    assert!(required_extensions.contains(&"VK_KHR_surface".to_string()));

    println!("Vulkan required extensions: {:?}", required_extensions);

    //Load up all the entry points using 0 as the VkInstance,
    //since you can't have an instance before you get vkCreateInstance...
    let mut entry_points: EntryPoints = EntryPoints::load(|func| {
        window.get_instance_proc_address(0, func.to_str().unwrap()) as *const c_void
    });

    let instance: VkInstance = unsafe { create_instance(&mut entry_points) };

    let mut instance_ptrs: InstancePointers = InstancePointers::load(|func| {
        window.get_instance_proc_address(instance, func.to_str().unwrap()) as *const c_void
    });

    //Load other pointers and do other Vulkan stuff here

    unsafe {
        destroy_instance(instance, &mut instance_ptrs);
    }

    println!("Vulkan instance successfully created and destroyed.");
}

unsafe fn create_instance(entry_points: &mut EntryPoints) -> VkInstance {
    let mut instance: VkInstance = mem::uninitialized();

    //This is literally the bare minimum required to create a blank instance
    //You'll want to fill in this with real data yourself
    let info: InstanceCreateInfo = InstanceCreateInfo {
        sType: vk::STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
        pNext: ptr::null(),
        flags: 0,
        pApplicationInfo: ptr::null(),
        enabledLayerCount: 0,
        ppEnabledLayerNames: ptr::null(),
        //These two should use the extensions returned by window.get_required_instance_extensions
        enabledExtensionCount: 0,
        ppEnabledExtensionNames: ptr::null()
    };

    let res: VkResult = entry_points.CreateInstance(&info as *const InstanceCreateInfo,
                                                    ptr::null(),
                                                    &mut instance as *mut VkInstance);

    assert_eq!(res, vk::SUCCESS);

    instance
}

unsafe fn destroy_instance(instance: VkInstance, instance_ptrs: &mut InstancePointers) {
    instance_ptrs.DestroyInstance(instance, ptr::null());
}