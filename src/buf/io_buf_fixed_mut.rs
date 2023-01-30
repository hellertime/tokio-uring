use super::IoBufMut;

/// A fixed, mutable `io_uring` compatible buffer.
///
/// The `IoBufFixedMut` trait is implemented by buffer types that can be used with
/// fixed io-uring operations. That is the buffers have been registered with the
/// `tokio-uring` runtime. Users will not need to use this trait directly.
///
/// `io_uring` limits the size of a fixed buffer to 1GiB.
///
/// # Safety
///
/// Buffers passed to fixed `io-uring` operations must reference a stable, registered memory
/// region. `IoBufFixedMut` inherits all the safety requirements of the `IoBufMut`.
///
/// In addition, the index returned by `buf_index` must be associated with the buffer otherwise
/// the behavior of the operations is undefined.
pub unsafe trait IoBufFixedMut: IoBufMut {
    /// Associates an index with the buffer returned by `IoBufMut::stable_mut_ptr(self)`.
    ///
    /// # Safety
    ///
    /// The caller must ensure this index is correctly associated with the buffer, for
    /// the `tokio-uring` runtime where the `IoBufFixedMut` is being used.
    unsafe fn set_buf_index(&mut self, index: u16);

    /// Return the index of of the registered buffer.
    fn buf_index(&self) -> u16;
}
