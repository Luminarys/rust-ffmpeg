use ffi::*;
use libc::{c_uchar, c_void, uint8_t, c_int, int64_t};

pub struct Context {
    pub cleanup: fn(*mut c_void),
    ptr: *mut AVIOContext,
}

extern "C" fn default_read(_opaque: *mut c_void,
                           _buffer: *mut uint8_t,
                           _buffer_len: c_int)
                           -> c_int {
    0
}
extern "C" fn default_write(_opaque: *mut c_void,
                            _buffer: *mut uint8_t,
                            _buffer_len: c_int)
                            -> c_int {
    0
}
extern "C" fn default_seek(_opaque: *mut c_void, _offset: int64_t, _whence: c_int) -> int64_t {
    0
}

fn default_cleanup(_opaque: *mut c_void) {}

impl Context {
    pub fn new<T>(buffer_size: usize,
                  writeable: bool,
                  data: T,
                  read_packet: Option<extern "C" fn(*mut c_void, *mut uint8_t, c_int) -> c_int>,
                  write_packet: Option<extern "C" fn(*mut c_void, *mut uint8_t, c_int) -> c_int>,
                  seek: Option<extern "C" fn(*mut c_void, int64_t, c_int) -> int64_t>,
                  cleanup: Option<fn(*mut c_void)>)
                  -> Context {
        unsafe {
            let buffer = av_malloc(buffer_size) as *mut c_uchar;
            let rf = match read_packet {
                Some(f) => f,
                None => default_read,
            };
            let wf = match write_packet {
                Some(f) => f,
                None => default_write,
            };
            let sf = match seek {
                Some(f) => f,
                None => default_seek,
            };
            let cf = match cleanup {
                Some(f) => f,
                None => default_cleanup,
            };
            let w = if writeable {
                1
            } else {
                0
            };
            let boxed = Box::new(data);
            let ptr = avio_alloc_context(buffer,
                                         buffer_size as i32,
                                         w,
                                         Box::into_raw(boxed) as *mut c_void,
                                         rf,
                                         wf,
                                         sf);
            Context {
                ptr: ptr,
                cleanup: cf,
            }
        }
    }

    pub unsafe fn as_ptr(&self) -> *const AVIOContext {
        self.ptr as *const _
    }

    pub unsafe fn as_mut_ptr(&mut self) -> *mut AVIOContext {
        self.ptr
    }
}
