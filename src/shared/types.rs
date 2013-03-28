/* Function pointer types */
// Will have to be changed once we can do external C callbacks nicely

pub type GLFWglproc             = *u8;  // typedef void (*GLFWglproc)(void);

pub type GLFWerrorfun           = *u8;  // typedef void (* GLFWerrorfun)(int,const char*);
pub type GLFWwindowposfun       = *u8;  // typedef void (* GLFWwindowposfun)(*GLFWwindow,int,int);
pub type GLFWwindowsizefun      = *u8;  // typedef void (* GLFWwindowsizefun)(*GLFWwindow,int,int);
pub type GLFWwindowclosefun     = *u8;  // typedef void (* GLFWwindowclosefun)(*GLFWwindow);
pub type GLFWwindowrefreshfun   = *u8;  // typedef void (* GLFWwindowrefreshfun)(*GLFWwindow);
pub type GLFWwindowfocusfun     = *u8;  // typedef void (* GLFWwindowfocusfun)(*GLFWwindow,int);
pub type GLFWwindowiconifyfun   = *u8;  // typedef void (* GLFWwindowiconifyfun)(*GLFWwindow,int);
pub type GLFWmousebuttonfun     = *u8;  // typedef void (* GLFWmousebuttonfun)(*GLFWwindow,int,int);
pub type GLFWcursorposfun       = *u8;  // typedef void (* GLFWcursorposfun)(*GLFWwindow,int,int);
pub type GLFWcursorenterfun     = *u8;  // typedef void (* GLFWcursorenterfun)(*GLFWwindow,int);
pub type GLFWscrollfun          = *u8;  // typedef void (* GLFWscrollfun)(*GLFWwindow,double,double);
pub type GLFWkeyfun             = *u8;  // typedef void (* GLFWkeyfun)(*GLFWwindow,int,int);
pub type GLFWcharfun            = *u8;  // typedef void (* GLFWcharfun)(*GLFWwindow,unsigned int);
pub type GLFWmonitorfun         = *u8;  // typedef void (* GLFWmonitorfun)(*GLFWmonitor,int);

/* Monitor handle type */
pub struct GLFWmonitor {}

/* Window handle type */
pub struct GLFWwindow {}

pub type GLFWgammaramp = ::hl::GammaRamp;
pub type GLFWvidmode   = ::hl::VidMode;