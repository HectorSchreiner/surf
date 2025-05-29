use thiserror::Error;

pub mod alerts;
pub mod users;
pub mod vulnerabilities;

// pub trait VulnerabilityFeed {
//     async fn list_vulnerabilities(&self) ->
// }

pub trait StringNormalize {
    fn normalize(&self) -> String;
}

pub trait StringSanitize {
    fn sanitize(&self) -> String;
}

impl StringNormalize for String {
    /// Returns a normalized version of the string.
    ///
    /// - Trims leading/trailing whitespace
    /// - Collapses consecutive whitespace into single spaces
    /// - Converts all characters to lowercase
    fn normalize(&self) -> String {
        self.trim()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
            .to_lowercase()
            .to_string()
    }
}

impl StringSanitize for String {
    /// Returns a sanitized version of the string, using some random sanitization libary.
    fn sanitize(&self) -> String {
        self.to_string()
    }
}

#[derive(Error, Debug, Clone, PartialEq, Eq)]
#[error("{0}: is not within the size constraints")]
pub struct ValidationError(usize);

pub trait ValidateLength {
    /// Minimum allowed length (inclusive)
    const MIN_LEN: usize;
    /// Maximum allowed length (inclusive)
    const MAX_LEN: usize;

    /// Checks if the string length is between `MIN_LEN` & `MAX_LEN`
    /// Returns a `Err(ValidationError(len))` if the string len is outide the range.
    fn validate_length(s: &str) -> Result<&str, ValidationError> {
        let len = s.chars().count();
        match (Self::MIN_LEN..=Self::MAX_LEN).contains(&len) {
            true => Ok(s),
            false => Err(ValidationError(len)),
        }
    }
}
