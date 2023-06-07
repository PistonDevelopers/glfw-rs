// Copyright 2013 The GLFW-RS Developers. For a full listing of the authors,
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

//! Private callback support functions.

macro_rules! callback (
    (
        type Args = ($($arg:ident: $arg_ty:ty),*);
        type Callback = $Callback:ident;
        let ext_set = $ext_set:expr;
        fn callback($($ext_arg:ident: $ext_arg_ty:ty),*) $call:expr
    ) => (
        thread_local!(static CALLBACK_KEY: RefCell<Option<Box<dyn Object<Args> + 'static>>> = RefCell::new(None));

        type Args = ($($arg_ty),*,);

        trait Object<T> {
            fn call(&self, args: T);
        }

        impl<UserData> Object<Args> for crate::Callback<fn($($arg_ty),*, &UserData), UserData> {
            fn call(&self, ($($arg),*,): Args) {
                (self.f)($($arg),*, &self.data);
            }
        }

        pub fn set<UserData: 'static>(f: crate::$Callback<UserData>) {
            let mut boxed_cb = Some(Box::new(f) as Box<dyn Object<Args> + 'static>);
            CALLBACK_KEY.with(|cb| {
                *cb.borrow_mut() = boxed_cb.take();
            });
            ($ext_set)(Some(callback as extern "C" fn($($ext_arg: $ext_arg_ty),*)));
        }

        pub fn unset() {
            CALLBACK_KEY.with(|cb| {
                *cb.borrow_mut() = None;
            });
            ($ext_set)(None);
        }

        extern "C" fn callback($($ext_arg: $ext_arg_ty),*) {
            CALLBACK_KEY.with(|cb| {
                match *cb.borrow() {
                    Some(ref cb) => unsafe { cb.call($call) },
                    _ => {}
                }
            })
        }
    )
);

pub mod error {
    use std::cell::RefCell;
    use std::mem;
    use std::os::raw::{c_char, c_int};

    callback!(
        type Args = (error: crate::Error, description: String);
        type Callback = ErrorCallback;
        let ext_set = |cb| unsafe { crate::ffi::glfwSetErrorCallback(cb) };
        fn callback(error: c_int, description: *const c_char) {
            (mem::transmute(error), crate::string_from_c_str(description))
        }
    );
}

pub mod monitor {
    use std::cell::RefCell;
    use std::mem;
    use std::os::raw::c_int;

    callback!(
        type Args = (monitor: crate::Monitor, event: crate::MonitorEvent);
        type Callback = MonitorCallback;
        let ext_set = |cb| unsafe { crate::ffi::glfwSetMonitorCallback(cb) };
        fn callback(monitor: *mut crate::ffi::GLFWmonitor, event: c_int) {
            let monitor = crate::Monitor {
                ptr: monitor
            };
            (monitor, mem::transmute(event))
        }
    );
}

pub mod joystick {
    use std::cell::RefCell;
    use std::mem;
    use std::os::raw::c_int;

    callback!(
        type Args = (joystick_id: crate::JoystickId, event: crate::JoystickEvent);
        type Callback = JoystickCallback;
        let ext_set = |cb| unsafe { crate::ffi::glfwSetJoystickCallback(cb) };
        fn callback(joystick_id: c_int, event: c_int) {
            (mem::transmute(joystick_id), mem::transmute(event))
        }
    );
}

pub mod unbuffered {
    use crate::{WindowEvent, WindowId};
    use std::cell::RefCell;

    type CallbackPtr = *mut std::ffi::c_void;
    type HandlerFn = fn(
        window_id: WindowId,
        event: (f64, WindowEvent),
        callback_ptr: CallbackPtr,
    ) -> Option<(f64, WindowEvent)>;

    thread_local! {
        static HANDLER: RefCell<Option<(HandlerFn, CallbackPtr)>> = RefCell::new(None);
    }

    pub struct UnsetHandlerGuard {
        _private: (),
    }

    impl Drop for UnsetHandlerGuard {
        fn drop(&mut self) {
            HANDLER.with(|ref_cell| {
                *ref_cell.borrow_mut() = None;
            })
        }
    }

    pub unsafe fn handle(
        window_id: WindowId,
        event: (f64, WindowEvent),
    ) -> Option<(f64, WindowEvent)> {
        HANDLER.with(|ref_cell| {
            if let Some((handler, callback_ptr)) = *ref_cell.borrow() {
                handler(window_id, event, callback_ptr)
            } else {
                Some(event)
            }
        })
    }

    pub unsafe fn set_handler<F>(callback: &mut F) -> UnsetHandlerGuard
    where
        F: FnMut(WindowId, (f64, WindowEvent)) -> Option<(f64, WindowEvent)>,
    {
        fn handler<F>(
            window_id: WindowId,
            event: (f64, WindowEvent),
            callback_ptr: CallbackPtr,
        ) -> Option<(f64, WindowEvent)>
        where
            F: FnMut(WindowId, (f64, WindowEvent)) -> Option<(f64, WindowEvent)>,
        {
            unsafe {
                let callback: &mut F = &mut *(callback_ptr as *mut F);
                callback(window_id, event)
            }
        }
        HANDLER.with(|ref_cell| {
            let callback_ptr = callback as *mut F as CallbackPtr;
            *ref_cell.borrow_mut() = Some((handler::<F>, callback_ptr));
        });
        UnsetHandlerGuard { _private: () }
    }
}