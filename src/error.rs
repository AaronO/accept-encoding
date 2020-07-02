use std::result;

/// A specialized [`Result`] type for this crate's operations.
///
/// This is generally used to avoid writing out [Error] directly and
/// is otherwise a direct mapping to [`Result`].
///
/// [`Result`]: https://doc.rust-lang.org/nightly/std/result/enum.Result.html
/// [`Error`]: std.struct.Error.html
pub type Result<T> = result::Result<T, Error>;

/// A list enumerating the categories of errors in this crate.
///
/// This list is intended to grow over time and it is not recommended to
/// exhaustively match against it.
///
/// It is used with the [`Error`] struct.
///
/// [`Error`]: std.struct.Error.html
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Invalid header encoding.
    #[error("Invalid header encoding.")]
    InvalidEncoding,
    /// The encoding scheme is unknown.
    #[error("Unknown encoding scheme.")]
    UnknownEncoding,
    /// Any error not part of this list.
    #[error("Generic error.")]
    Other,
}
