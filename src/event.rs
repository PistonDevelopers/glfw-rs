macro_rules! event(
    (
        mod $mod_id:ident {
            extern_callback_type = $ext_cbfun:ty;
            intern_callback_type = $cbfun:ty;
            
            extern_params {
                $( $arg:ident : $arg_ty:ty => { $arg_conv:expr } ),+
            }
            
            message_struct {
                $($field:ident : $field_ty:ty => { $field_conv:expr } ),+
            }
        }
    ) => (
        mod $mod_id {
            /**
             * A key for setting and retrieving the callback from task-
             * local storage
             */
            fn tls_key(_: @$cbfun) {}
            
            /**
             * Stores the callback in task-local storage and then calls
             * `f` with  with `extfun` as the argument.
             */
            pub fn set_callback(cbfun: $cbfun, f: &fn($ext_cbfun) ) {
                unsafe {
                    task::local_data::local_data_set(tls_key, @cbfun);
                    f(ext_cbfun);
                }
            }
            
            /**
             * An external function that invokes the callback currently stored
             * in task-local storage, if it exists.
             */
            pub extern fn ext_cbfun( $($arg : $arg_ty),+ ) {
                unsafe {
                    do task::local_data::local_data_get(tls_key).map |&cb| {
                        (*cb)( $($arg_conv),+ )
                    };
                }
            }
            
            pub struct Msg { $($field : $field_ty),+ }
            
            pub type Port = comm::Port<Msg>;
            
            pub fn spawn_listener(f: ~fn(Msg)) -> $cbfun {
                let (port, chan) = comm::stream();
                
                // call the supplied function every time an event message is recieved
                do task::spawn {
                    loop { f(port.recv()); }
                }
                
                // return a callback that sends a message to the channel
                return |$($arg),+| {
                    chan.send(Msg { $($field: $field_conv),+ });
                };
            }
        }
    )
)


// Error Event Module
pub event!(
    mod error {
        extern_callback_type = ::api::GLFWerrorfun;
        intern_callback_type = ::ErrorFun;
        
        extern_params {
            err:    libc::c_int             => { err },
            format: *libc::c_char           => { str::raw::from_c_str(format) }
        }
        
        message_struct {
            err:    libc::c_int             => { err },
            format: ~str                    => { format }
        }
    }
)

// Monitor Event Module
pub event!(
    mod monitor {
        extern_callback_type = ::api::GLFWmonitorfun;
        intern_callback_type = ::MonitorFun;
        
        extern_params {
            monitor:    *::api::GLFWmonitor => { &::Monitor(monitor) },
            event:      libc::c_int         => { event }
        }
        
        message_struct {
            monitor:    ::Monitor           => { *monitor },
            event:      libc::c_int         => { event }
        }
    }
)

// Window Position Event Module
pub event!(
    mod windowpos {
        extern_callback_type = ::api::GLFWwindowposfun;
        intern_callback_type = ::WindowPosFun;
        
        extern_params {
            window:     *::api::GLFWwindow  => { &::Window(window) },
            x:          libc::c_int         => { x as int },
            y:          libc::c_int         => { y as int }
        }
        
        message_struct {
            window:     ::Window            => { *window },
            x:          int                 => { x },
            y:          int                 => { y }
        }
    }
)

// Window Size Event Module
pub event!(
    mod windowsize {
        extern_callback_type = ::api::GLFWwindowsizefun;
        intern_callback_type = ::WindowSizeFun;
        
        extern_params {
            window:     *::api::GLFWwindow  => { &::Window(window) },
            width:      libc::c_int         => { width as int },
            height:     libc::c_int         => { height as int }
        }
        
        message_struct {
            window:     ::Window            => { *window },
            width:      int                 => { width },
            height:     int                 => { height }
        }
    }
)

// Window Close Event Module
pub event!(
    mod windowclose {
        extern_callback_type = ::api::GLFWwindowclosefun;
        intern_callback_type = ::WindowCloseFun;
        
        extern_params {
            window:     *::api::GLFWwindow  => { &::Window(window) }
        }
        
        message_struct {
            window:     ::Window            => { *window }
        }
    }
)

// Window Refresh Event Module
pub event!(
    mod windowrefresh {
        extern_callback_type = ::api::GLFWwindowrefreshfun;
        intern_callback_type = ::WindowRefreshFun;
        
        extern_params {
            window:     *::api::GLFWwindow  => { &::Window(window) }
        }
        
        message_struct {
            window:     ::Window            => { *window }
        }
    }
)

// Window Focus Event Module
pub event!(
    mod windowfocus {
        extern_callback_type = ::api::GLFWwindowfocusfun;
        intern_callback_type = ::WindowFocusFun;
        
        extern_params {
            window:     *::api::GLFWwindow  => { &::Window(window) },
            activated:  libc::c_int         => { activated as bool }
        }
        
        message_struct {
            window:     ::Window            => { *window },
            activated:  bool                => { activated }
        }
    }
)

// Window Iconify Event Module
pub event!(
    mod windowiconify {
        extern_callback_type = ::api::GLFWwindowiconifyfun;
        intern_callback_type = ::WindowIconifyFun;
        
        extern_params {
            window:     *::api::GLFWwindow  => { &::Window(window) },
            iconified:  libc::c_int         => { iconified as bool }
        }
        
        message_struct {
            window:     ::Window            => { *window },
            iconified:  bool                => { iconified }
        }
    }
)

// Key Event Module
pub event!(
    mod key {
        extern_callback_type = ::api::GLFWkeyfun;
        intern_callback_type = ::KeyFun;
        
        extern_params {
            window:     *::api::GLFWwindow  => { &::Window(window) },
            key:        libc::c_int         => { key },
            action:     libc::c_int         => { action }
        }
        
        message_struct {
            window:     ::Window            => { *window },
            key:        libc::c_int         => { key },
            action:     libc::c_int         => { action }
        }
    }
)

// Character Event Module
pub event!(
    mod char {
        extern_callback_type = ::api::GLFWcharfun;
        intern_callback_type = ::CharFun;
        
        extern_params {
            window:     *::api::GLFWwindow  => { &::Window(window) },
            character:  libc::c_uint        => { character as char }
        }
        
        message_struct {
            window:     ::Window            => { *window },
            character:  char                => { character }
        }
    }
)

// Mouse Button Event Module
pub event!(
    mod mousebutton {
        extern_callback_type = ::api::GLFWmousebuttonfun;
        intern_callback_type = ::MouseButtonFun;
        
        extern_params {
            window:     *::api::GLFWwindow  => { &::Window(window) },
            button:     libc::c_int         => { button },
            action:     libc::c_int         => { action }
        }
            
        message_struct {
            window:     ::Window            => { *window },
            button:     libc::c_int         => { button },
            action:     libc::c_int         => { action }
        }
    }
)

// Cursor Position Event Module
pub event!(
    mod cursorpos {
        extern_callback_type = ::api::GLFWscrollfun;
        intern_callback_type = ::CursorPosFun;
        
        extern_params {
            window:     *::api::GLFWwindow  => { &::Window(window) },
            x:          libc::c_int         => { x as int },
            y:          libc::c_int         => { y as int }
        }
        
        message_struct {
            window:     ::Window            => { *window },
            x:          int                 => { x },
            y:          int                 => { y }
        }
    }
)

// Cursor Enter Event Module
pub event!(
    mod cursorenter {
        extern_callback_type = ::api::GLFWcursorposfun;
        intern_callback_type = ::CursorEnterFun;
        
        extern_params {
            window:     *::api::GLFWwindow  => { &::Window(window) },
            entered:    libc::c_int         => { entered as bool }
        }
        
        message_struct {
            window:     ::Window            => { *window },
            entered:    bool                => { entered }
        }
    }
)

// Scroll Event Module
pub event!(
    mod scroll {
        extern_callback_type = ::api::GLFWscrollfun;
        intern_callback_type = ::ScrollFun;
        
        extern_params {
            window:     *::api::GLFWwindow  => { &::Window(window) },
            x:          libc::c_double      => { x as f64 },
            y:          libc::c_double      => { y as f64 }
        }
        
        message_struct {
            window:     ::Window            => { *window },
            x:          f64                 => { x },
            y:          f64                 => { y }
        }
    }
)