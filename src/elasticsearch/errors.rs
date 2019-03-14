use crate::elasticsearch::models::Index;
use reqwest::Url;
use std::fmt;

#[derive(Eq, PartialEq)]
pub enum EsError {
    HostUnreachable(Url),
    IndexNotFound(Index, Option<Index>),
    Timeout,
}

impl fmt::Debug for EsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            EsError::HostUnreachable(url) => writeln!(f, "Host {} is unreachable", url),
            EsError::IndexNotFound(index, suggestion) => {
                writeln!(f, "\n\tIndex not found:   {}", index.name)?;
                if let Some(suggestion) = suggestion {
                    writeln!(f, "\tPerhaps you meant: {}", suggestion.name)?;
                }
                Ok(())
            }
            EsError::Timeout => writeln!(f, ""),
        }
    }
}
