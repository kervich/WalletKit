use std::fmt;

#[derive(Debug)]
pub enum Error {
    InvalidData { description: String },
    InvalidDerivationPath { description: String },
    InvalidKeystore,
    SuiError { description: String },
}

// Error

impl Error {
    pub fn reason(&self) -> String {
        match self {
            Error::InvalidData { description } => description.clone(),
            Error::InvalidDerivationPath { description } => description.clone(),
            Error::InvalidKeystore => "Invalid keystore".to_string(),
            Error::SuiError { description } => description.clone(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.reason())
    }
}


// // Result

// pub type Result<T> = std::result::Result<T, Error>;