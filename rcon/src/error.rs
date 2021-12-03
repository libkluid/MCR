use std::io::Error as IOError;

pub enum Error {
    IO(IOError),
    InvalidPassword,
    InvalidPacketLength,
    Unauthorised,
    Decode(&'static str, usize, usize),
}

impl From<IOError> for Error {
    fn from(err: IOError) -> Self {
        Error::IO(err)
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Decode(type_name, expected, found) => {
                writeln!(f,"To decode {} expected {} bytes, but found {} bytes.",
                    type_name,
                    expected,
                    found)
            }
            _ => std::fmt::Debug::fmt(self, f),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl std::error::Error for Error {}
