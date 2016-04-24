use std::error;
use std::result;
use std::fmt;
use std::io;
use std::num::{ParseIntError, ParseFloatError};
use std::str::ParseBoolError;

/// A specialized `Result` type for this crate.
///
/// Heavily borrowed from `std::io::Result` and family.
pub type Result<T> = result::Result<T, Error>;

/// The error type for this crate.
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    error: Box<error::Error+Send+Sync>,
}

/// General category of errors.
///
/// This list is intended to grow over time and it is not recommended to
/// exhaustively match against it.
//
// If a new category appears useful it should be added to this enum
#[derive(Copy, PartialEq, Eq, Clone, Debug)]
pub enum ErrorKind {
    /// Usually an `io::Error`.
    Io,
    /// Usually an `num::Parse*Error`.
    Parse,
    /// A protocol error communicating with a subprocess AI
    AiProtocol,
    /// An API yielded a result different than expected.
    Inconsistent,
    /// Any other error category.
    Other,
    /// Reserved for future kinds.
    #[doc(hidden)]
    __Nonexhaustive,
}

impl Error {
    /// Create a new error.
    ///
    /// # Examples
    ///
    /// ```
    /// use roboime_next::{Error, ErrorKind};
    ///
    /// // errors can be created from strings
    /// let custom_error = Error::new(ErrorKind::Other, "oh no!");
    ///
    /// // errors can also be created from other errors
    /// let custom_error2 = Error::new(ErrorKind::Io, custom_error);
    /// ```
    pub fn new<E>(kind: ErrorKind, error: E) -> Error
        where E: Into<Box<error::Error+Send+Sync>>
    {
        Error {
            kind: kind,
            error: error.into(),
        }
    }

    /// Consumes the `Error`, returning its inner error (if any).
    pub fn into_inner(self) -> Box<error::Error+Send+Sync>  {
        self.error
    }

    /// Returns the corresponding `ErrorKind` for this error.
    pub fn kind(&self) -> ErrorKind {
        self.kind
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.error.fmt(fmt)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        self.error.description()
    }

    fn cause(&self) -> Option<&error::Error> {
        self.error.cause()
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::new(ErrorKind::Io, err)
    }
}

impl From<ParseFloatError> for Error {
    fn from(err: ParseFloatError) -> Self {
        Error::new(ErrorKind::Parse, err)
    }
}

impl From<ParseIntError> for Error {
    fn from(err: ParseIntError) -> Self {
        Error::new(ErrorKind::Parse, err)
    }
}

impl From<ParseBoolError> for Error {
    fn from(err: ParseBoolError) -> Self {
        Error::new(ErrorKind::Parse, err)
    }
}

#[cfg(test)]
mod test {
    use super::{Error, ErrorKind};
    use std::error;
    use std::error::Error as error_Error;
    use std::fmt;

    #[test]
    fn test_downcasting() {
        #[derive(Debug)]
        struct TestError;

        impl fmt::Display for TestError {
            fn fmt(&self, _fmt: &mut fmt::Formatter) -> fmt::Result {
                Ok(())
            }
        }

        impl error::Error for TestError {
            fn description(&self) -> &str {
                "asdf"
            }
        }

        // we have to call all of these UFCS style right now since method
        // resolution won't implicitly drop the Send+Sync bounds
        let err = Error::new(ErrorKind::Other, TestError);
        assert_eq!(err.kind(), ErrorKind::Other);
        //assert!(err.get_ref().unwrap().is::<TestError>());
        assert_eq!("asdf", err.description());
        //assert!(err.get_mut().unwrap().is::<TestError>());
        let extracted = err.into_inner();
        extracted.downcast::<TestError>().unwrap();
    }
}
