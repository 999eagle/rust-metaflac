extern crate byteorder;

use std::error;
use std::io;
use std::fmt;
use std::string;

/// Type alias for the result of tag operations.
pub type Result<T> = ::std::result::Result<T, Error>;

/// Kinds of errors that may occur while performing metadata operations.
#[derive(Debug)]
pub enum ErrorKind {
    /// An error kind indicating that an IO error has occurred. Contains the original io::Error.
    Io(io::Error),
    /// An error kind indicating that an error occured while parsing data to bytes. Contains the
    /// original byteorder::Error.
    ByteOrder(byteorder::Error),
    /// An error kind indicating that a string decoding error has occurred. Contains the invalid
    /// bytes.
    StringDecoding(string::FromUtf8Error),
    /// An error kind indicating that some input was invalid.
    InvalidInput,
}

/// A structure able to represent any error that may occur while performing metadata operations.
pub struct Error {
    /// The kind of error.
    pub kind: ErrorKind,
    /// A human readable string describing the error.
    pub description: &'static str,
}

impl Error {
    /// Creates a new `Error` using the error kind and description.
    pub fn new(kind: ErrorKind, description: &'static str) -> Error {
        Error { kind: kind, description: description }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        if self.cause().is_some() {
            self.cause().unwrap().description()
        } else {
           match self.kind {
               ErrorKind::Io(ref err) => error::Error::description(err),
               ErrorKind::ByteOrder(ref err) => err.description(),
               ErrorKind::StringDecoding(ref err) => err.description(),
               _ => self.description
           }
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match self.kind {
            ErrorKind::Io(ref err) => Some(err),
            ErrorKind::ByteOrder(ref err) => Some(err),
            ErrorKind::StringDecoding(ref err) => Some(err), 
            _ => None 
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error { kind: ErrorKind::Io(err), description: "" }
    }
}

impl From<string::FromUtf8Error> for Error {
    fn from(err: string::FromUtf8Error) -> Error {
        Error { kind: ErrorKind::StringDecoding(err), description: "" }
    }
}

impl From<byteorder::Error> for Error {
    fn from(err: byteorder::Error) -> Error {
        Error { kind: ErrorKind::ByteOrder(err), description: "" }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        if self.description != "" {
            write!(out, "{:?}: {}", self.kind, error::Error::description(self))
        } else {
            write!(out, "{}", error::Error::description(self))
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        if self.description != "" {
            write!(out, "{:?}: {}", self.kind, error::Error::description(self))
        } else {
            write!(out, "{}", error::Error::description(self))
        }
    }
}
