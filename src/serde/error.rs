#[derive(Debug)]
pub enum Error {
    Message(String),
    ParseError,
    EOF,
}

impl std::fmt::Display for Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str(match *self {
            Error::Message(ref s) => s,
            Error::ParseError => "Parse Error",
            Error::EOF => "EOF",
        })
    }
}

impl std::error::Error for Error {}

impl serde::de::Error for Error {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl std::convert::From<nom::Err<()>> for Error {
    fn from(_v: nom::Err<()>) -> Error {
        Error::ParseError
    }
}

pub type Result<T> = std::result::Result<T, Error>;
