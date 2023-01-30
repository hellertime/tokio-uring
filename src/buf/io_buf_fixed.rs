use super::IoBuf;

/// A fixed `io-uring` compatible buffer.
///
/// The `IoBuf` trait is implemented by buffer types that can be used with
/// fixed io-uring operations. Users will not need to use this trait directly.
/// The [`BoundedBufFixed`] trait provides some useful methods including `slice`.
///
/// # Safety
///
/// Buffers passed to fixed `io-uring` operations must reference a stable, registered memory
/// region. `IoBufFixed` inherits all the safety requirements of the `IoBuf`.
///
/// In addition, the index returned by `buf_index` must be associated with the buffer otherwise
/// the behavior of the operations is undefined.
///
/// [`BoundedBuf`]: crate::buf::BoundedBuf
pub unsafe trait IoBufFixed: IoBuf {
    /// Return the index of the registered buffer.
    fn buf_index(&self) -> u16;
}
