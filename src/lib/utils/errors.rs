#![allow(unused)]
use std::io;

pub enum CocapError {
    IoError(io::Error),
    SerializationError(serde_json::Error),
    MissingConfigFiles,
    InvalidConfiguration,
}

pub type CocapResult<T> = Result<T, CocapError>;

impl std::fmt::Display for CocapError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use CocapError::*;
        match &self {
            IoError(ref e) => e.fmt(f),
            SerializationError(ref e) => e.fmt(f),
            MissingConfigFiles => f.write_str("Missing config files for cocap"),
            InvalidConfiguration => f.write_str("Invalid cocap configuration found"),
        }
    }
}

impl std::fmt::Debug for CocapError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use CocapError::*;
        match &self {
            IoError(ref e) => e.fmt(f),
            SerializationError(ref e) => e.fmt(f),
            MissingConfigFiles => f.write_str("Missing config files for cocap"),
            InvalidConfiguration => f.write_str("Invalid cocap configuration found"),
        }
    }
}

impl From<io::Error> for CocapError {
    fn from(e: io::Error) -> Self {
        CocapError::IoError(e)
    }
}

impl From<serde_json::Error> for CocapError {
    fn from(e: serde_json::Error) -> Self {
        CocapError::SerializationError(e)
    }
}
