use std::{
    fmt::{write, Display, Formatter, Result},
    io::Error,
};

#[derive(Debug)]
pub enum Errornumber {
    ErrorNotNumber(String),
    IOError(Error),

}

impl Display for Errornumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self {
            Errornumber::IOError(a) => write!(f, "there was an error in the system: {a}"),
            Errornumber::ErrorNotNumber(a) => write!(f, "you have to give us a number: {a}"),
  
        }
    }
}

impl std::error::Error for Errornumber {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Errornumber::IOError(ref err) => Some(err),
            _ => None,
        }
    }
}





impl From<std::io::Error> for Errornumber {
    fn from(value: std::io::Error) -> Self {
        Errornumber::IOError(value)
    }
}
