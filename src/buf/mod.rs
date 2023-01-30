//! Utilities for working with buffers.
//!
//! `io-uring` APIs require passing ownership of buffers to the runtime. The
//! crate defines [`IoBuf`] and [`IoBufMut`] traits which are implemented by buffer
//! types that respect the `io-uring` contract. In addition the [`IoBufFixedMut`] trait
//! can represent a buffer which has been registered with the `tokio-uring` runtime.

pub mod fixed;

mod io_buf;
pub use io_buf::IoBuf;

mod io_buf_fixed;
pub use io_buf_fixed::IoBufFixed;

mod io_buf_mut;
pub use io_buf_mut::IoBufMut;

mod io_buf_fixed_mut;
pub use io_buf_fixed_mut::IoBufFixedMut;

mod slice;
pub use slice::Slice;

mod bounded;
pub use bounded::{BoundedBuf, BoundedBufFixed, BoundedBufFixedMut, BoundedBufMut};

pub(crate) fn deref(buf: &impl IoBuf) -> &[u8] {
    // Safety: the `IoBuf` trait is marked as unsafe and is expected to be
    // implemented correctly.
    unsafe { std::slice::from_raw_parts(buf.stable_ptr(), buf.bytes_init()) }
}

pub(crate) fn deref_mut(buf: &mut impl IoBufMut) -> &mut [u8] {
    // Safety: the `IoBufMut` trait is marked as unsafe and is expected to be
    // implemented correct.
    unsafe { std::slice::from_raw_parts_mut(buf.stable_mut_ptr(), buf.bytes_init()) }
}
