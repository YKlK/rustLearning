use std::{
    fmt::{write, Display, Formatter, Result},
    io::Error,
};

use calamine::{Xlsx, XlsxError};
use xlsxwriter:: XlsxError as XlsxWriterError;
#[derive(Debug)]
pub enum Errornumber {
    ErrorNotNumber(String),
    IOError(Error),
    CalamineError(XlsxError),
    Xlsxwriter(XlsxWriterError)
}

impl Display for Errornumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self {
            Errornumber::IOError(a) => write!(f, "there was an error in the system: {a}"),
            Errornumber::ErrorNotNumber(a) => write!(f, "you have to give us a number: {a}"),
            Errornumber::CalamineError(a)=>write!(f,"you have and error with XLSX: {a}"),
            Errornumber::Xlsxwriter(a)=>write!(f,"you have and error with XLSXWriter: {a}")
        }
    }
}

impl std::error::Error for Errornumber {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Errornumber::IOError(ref err) => Some(err),
            Errornumber::Xlsxwriter(ref err) => Some(err),
            _ => None,
        }
    }
}


impl From<calamine::XlsxError> for Errornumber {
    fn from(value: calamine::XlsxError) -> Self {
        Errornumber::CalamineError(value)
    }
}


impl From<std::io::Error> for Errornumber {
    fn from(value: std::io::Error) -> Self {
        Errornumber::IOError(value)
    }
}
impl From<XlsxWriterError> for Errornumber {
    fn from(err: XlsxWriterError) -> Errornumber {
        Errornumber::Xlsxwriter(err)
    }
}