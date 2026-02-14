use std::fmt;

#[derive(Debug)]
pub enum Error {
    AlloyError { description: String },
    MnemonicError { description: String },
    NotImplemented,
    SuiError { description: String },
}

impl Error {
    pub fn reason(&self) -> String {
        match self {
            Error::AlloyError { description } => description.clone(),
            Error::MnemonicError { description } => description.clone(),
            Error::NotImplemented => "Not implemented".to_string(),
            Error::SuiError { description } => description.clone(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.reason())
    }
}