use thiserror::Error;

pub mod users;
pub mod vulnerabilities;
pub mod alerts;

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
    fn normalize(&self) -> String {
        self
            .trim()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
            .to_lowercase()
            .to_string()
    }
}

impl StringSanitize for String {
    fn sanitize(&self) -> String {
        self.to_string()
    }
}

#[derive(Error, Debug, Clone, PartialEq, Eq)]
#[error("{0}: is not within the size constraints")]
pub struct ValidationError(usize);
pub trait ValidateLength {
    const MIN_LEN: usize;
    const MAX_LEN: usize;

    fn validate_length(s: &str) -> Result<&str, ValidationError> {
        let len = s.chars().count();
        match (Self::MIN_LEN..=Self::MAX_LEN).contains(&len) {
            true => Ok(s),
            false => Err(ValidationError(len))
        }
    }
}