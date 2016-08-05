use ffi::*;
use libc::{c_uchar, c_void, uint8_t, c_int};
use std::ptr;

pub struct Context {
    ctx: *mut AVIOContext,
}

impl Context {
    fn new<T, F, S> (buffer_size: usize,
                          writeable: bool,
                          data: T,
                          read_packet: Option<F>,
                          write_packet: Option<F>,
                          seek: Option<S>)
                          -> Context
        where F: Fn(*mut c_void, *mut uint8_t, c_int) -> c_int,
              S: Fn(*mut c_void, *mut uint8_t, c_int) -> c_int
    {
        unsafe {
            let buffer = av_malloc(buffer_size) as *mut c_uchar;
		    let rf   = ptr::null_mut();
		    let wf   = ptr::null_mut();
		    let sf   = ptr::null_mut();
            let w = if writeable { 1 } else { 0 };
            let boxed = Box::new(data);
            let ctx = avio_alloc_context(buffer, buffer_size as i32, w, Box::into_raw(boxed) as *mut c_void, read_packet.unwrap(), write_packet.unwrap(), seek.unwrap());
            Context {
                ctx: ctx,
            }
        }
    }
}
