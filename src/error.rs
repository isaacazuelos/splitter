use calamine;

#[derive(Debug)]
pub enum Error {
    IO(::std::io::Error),
    Excel(calamine::Error),
    InvalidExtension,
    SheetMissing(String),
}

impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            Error::IO(ref e) => write!(f, "io error: {}", e),
            Error::IO(ref e) => write!(f, "excel error: {}", e),
            InvalidExtension => write!(f, "input file is not of a supported file type"),
            SheetMissing(ref s) => write!(f, "input file does not have a sheet named '{}'", s),
        }
    }
}

impl ::std::error::Error for Error {
    fn cause(&self) -> Option<&::std::error::Error> {
        match self {
            Error::IO(ref e) => Some(e),
            Error::Excel(ref e) => Some(e),
            _ => None,
        }
    }
}

impl From<calamine::Error> for Error {
    fn from(err: calamine::Error) -> Self {
        Error::Excel(err)
    }
}

impl From<::std::io::Error> for Error {
    fn from(err: ::std::io::Error) -> Self {
        Error::IO(err)
    }
}
