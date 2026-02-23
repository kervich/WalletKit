use std::fmt;

#[derive(Debug)]
pub enum Error {
    AlloyError { description: String },
    InvalidAddress { description: String },
    MnemonicError { description: String },
    NotImplemented,
    RuntimeError { description: String },
    SuiError { description: String },
    TrezorError { description: String }
}

impl Error {
    pub fn reason(&self) -> String {
        match self {
            Error::AlloyError { description } => description.clone(),
            Error::InvalidAddress { description } => description.clone(),
            Error::MnemonicError { description } => description.clone(),
            Error::NotImplemented => "Not implemented".to_string(),
            Error::RuntimeError { description } => description.clone(),
            Error::SuiError { description } => description.clone(),
            Error::TrezorError { description } => description.clone()
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.reason())
    }
}

impl From<tokio::task::JoinError> for Error {
    fn from(e: tokio::task::JoinError) -> Self {
        Error::RuntimeError { description: e.to_string() }
    }
}