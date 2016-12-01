use ffi::*;
use libc::c_void;

#[derive(Copy, Clone, Debug)]
pub enum Mode {
    Input,
    Output,
    InputCIO(fn(*mut c_void)),
    OutputCIO(fn(*mut c_void)),
}

pub struct Destructor {
    ptr: *mut AVFormatContext,
    mode: Mode,
}

impl Destructor {
    pub unsafe fn new(ptr: *mut AVFormatContext, mode: Mode) -> Self {
        Destructor {
            ptr: ptr,
            mode: mode,
        }
    }
}

impl Drop for Destructor {
    fn drop(&mut self) {
        unsafe {
            match self.mode {
                Mode::Input => avformat_close_input(&mut self.ptr),

                Mode::Output => avformat_free_context(self.ptr),

                Mode::InputCIO(f) => {
                    f((*(*self.ptr).pb).opaque);
                    avformat_close_input(&mut self.ptr);
                }

                Mode::OutputCIO(f) => {
                    f((*(*self.ptr).pb).opaque);
                    avformat_free_context(self.ptr);
                }
            }
        }
    }
}
