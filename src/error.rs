use std::error;
use std::result;
use std::fmt;

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
    /// An OS Synchronization error, like `std::sync::PoisonError` or `std::sync::mpsc::RecvError`
    Sync,
    /// A system error, like USB errors from libusb
    System,
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

mod froms {
    use std::any::Any;
    use std::env::VarError;
    use std::io::Error as IoError;
    use std::net::AddrParseError;
    use std::num::{ParseIntError, ParseFloatError};
    use std::str::ParseBoolError;
    use std::sync::PoisonError;
    use std::sync::mpsc::{RecvError, SendError};
    use log::SetLoggerError;
    #[cfg(feature="usb-transceiver")]
    use libusb::Error as LibusbError;
    use protocol::ProtobufError;
    use ::error::{Error, ErrorKind};

    impl From<IoError> for Error {
        fn from(err: IoError) -> Self {
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

    impl<T> From<PoisonError<T>> for Error {
        fn from(_err: PoisonError<T>) -> Self {
            // XXX: the error here is not forwarded because it references to !Send !Sync data, more
            // over it is only an error::Error if T: Reflect, which is limited for our usage
            Error::new(ErrorKind::Sync, "lock poison error")
        }
    }

    impl From<RecvError> for Error {
        fn from(err: RecvError) -> Self {
            Error::new(ErrorKind::Sync, err)
        }
    }

    impl<T: Any + Send + Sync> From<SendError<T>> for Error {
        fn from(err: SendError<T>) -> Self {
            Error::new(ErrorKind::Sync, err)
        }
    }

    impl From<ProtobufError> for Error {
        fn from(err: ProtobufError) -> Self {
            Error::new(ErrorKind::Parse, err)
        }
    }

    impl From<AddrParseError> for Error {
        fn from(err: AddrParseError) -> Self {
            Error::new(ErrorKind::Parse, err)
        }
    }

    impl From<SetLoggerError> for Error {
        fn from(err: SetLoggerError) -> Self {
            Error::new(ErrorKind::Other, err)
        }
    }

    impl From<VarError> for Error {
        fn from(err: VarError) -> Self {
            Error::new(ErrorKind::Other, err)
        }
    }

    impl<T: Any + Send + ?Sized> From<Box<T>> for Error {
        fn from(_err: Box<T>) -> Self {
            // FIXME: make use of the actual error, something like:
            //use std::error::Error as ErrorError;
            //match (*err).downcast_ref::<ErrorError>() {
            //    Some(e) => Error::new(ErrorKind::Other, Box::new(e)),
            //    None => Error::new(ErrorKind::Other, "unknown"),
            //}
            Error::new(ErrorKind::Other, "unknown")
        }
    }

    #[cfg(feature="usb-transceiver")]
    impl From<LibusbError> for Error {
        fn from(err: LibusbError) -> Self {
            Error::new(ErrorKind::System, err)
        }
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
