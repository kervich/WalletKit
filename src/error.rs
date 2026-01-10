use std::fmt;

#[derive(Debug)]
pub enum Error {
    InvalidData { description: String },
    InvalidDerivationPath { description: String },
    InvalidKeystore,
    InvalidMnemonic { description: String },
    InvalidSeedLength { description: String },
    NotImplemented,
    SuiError { description: String },
}

impl Error {
    pub fn invalid_seed_length(length: usize) -> Self {
        Error::InvalidSeedLength {
            description: format!("Invalid seed length: {}", length),
        }
    }
}

// Error

impl Error {
    pub fn reason(&self) -> String {
        match self {
            Error::InvalidData { description } => description.clone(),
            Error::InvalidDerivationPath { description } => description.clone(),
            Error::InvalidKeystore => "Invalid keystore".to_string(),
            Error::InvalidMnemonic { description } => description.clone(),
            Error::InvalidSeedLength { description } => description.clone(),
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


// // Result

// pub type Result<T> = std::result::Result<T, Error>;