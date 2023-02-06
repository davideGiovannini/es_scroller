use crate::elasticsearch::models::Index;
use reqwest::Url;
use std::fmt;

use std::io;

pub enum EsError {
    HostUnreachable(Url),
    IndexNotFound(Index, Option<Index>),
    Timeout,
    Io(io::Error),
    JsonDeserialization(serde_json::Error),
}

impl fmt::Debug for EsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::HostUnreachable(url) => writeln!(f, "Host {url} is unreachable"),
            Self::IndexNotFound(index, suggestion) => {
                writeln!(f, "\n\tIndex not found:   {}", index.name)?;
                if let Some(suggestion) = suggestion {
                    writeln!(f, "\tPerhaps you meant: {}", suggestion.name)?;
                }
                Ok(())
            }
            Self::Timeout => writeln!(f),
            Self::Io(err) => writeln!(f, "{err}"),
            Self::JsonDeserialization(err) => writeln!(f, "invalid json\n\t{err}"),
        }
    }
}

impl From<io::Error> for EsError {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<serde_json::Error> for EsError {
    fn from(err: serde_json::Error) -> Self {
        Self::JsonDeserialization(err)
    }
}
