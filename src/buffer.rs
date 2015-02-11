
use core::prelude::*;

use alloc::heap::{EMPTY, allocate, deallocate};
use core::mem;
use core::nonzero::NonZero;
use core::num::Int;
use core::ptr;
use core::raw::Slice as RawSlice;

// Buffer that owns some fixed amount of data.
// Much of this is shamelessly borrowed from the Vec<T> implementation.
#[unsafe_no_drop_flag]
pub struct Buffer<T> {
    ptr: NonZero<*mut T>,
    len: usize,
}

// Safe to send/sync if the contained data is.
unsafe impl<T: Send> Send for Buffer<T> { }
unsafe impl<T: Sync> Sync for Buffer<T> { }

impl<T> Buffer<T> {
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }
    #[inline]
    pub fn empty() -> Buffer<T> {
        Buffer { ptr: unsafe { NonZero::new(EMPTY as *mut T) }, len: 0 }
    }
    #[inline]
    pub fn new(len: usize) -> Buffer<T> {
        if mem::size_of::<T>() == 0 {
            Buffer { ptr: unsafe { NonZero::new(EMPTY as *mut T) }, len: len }
        } else if len == 0 {
            Buffer::empty()
        } else {
            let size = len.checked_mul(mem::size_of::<T>())
                          .expect("Buffer size overflow");
            let ptr = unsafe { allocate(size, mem::min_align_of::<T>()) };
            if ptr.is_null() { ::alloc::oom() }
            Buffer { ptr: unsafe { NonZero::new(ptr as *mut T) }, len: len }
        }
    }
    #[inline]
    fn as_slice(&self) -> &[T] {
        unsafe {
            mem::transmute(RawSlice {
                data: *self.ptr,
                len: self.len
            })
        }
    }
}

#[unsafe_destructor]
impl<T> Drop for Buffer<T> {
    fn drop(&mut self) {
        // This is (and should always remain) a no-op if the fields are
        // zeroed (when moving out, because of #[unsafe_no_drop_flag]).
        if self.len != 0 {
            unsafe {
                for x in self.as_slice() {
                    ptr::read(x);
                }
                dealloc(*self.ptr, self.len)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Buffer;
    #[test]
    fn empty_buffer_is_empty() {
        let buf = Buffer::<u8>::empty();
        assert_eq!(buf.len(), 0);
    }
    #[test]
    fn new_buffer_has_correct_size() {
        let buf = Buffer::<u8>::new(2);
        assert_eq!(buf.len(), 2);
    }
}

#[inline]
unsafe fn dealloc<T>(ptr: *mut T, len: usize) {
    if mem::size_of::<T>() != 0 {
        deallocate(ptr as *mut u8,
                   len * mem::size_of::<T>(),
                   mem::min_align_of::<T>())
    }
}
