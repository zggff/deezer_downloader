use block_modes::BlockModeError;
use serde_json::{Map, Value};
use std::error::Error;

/// This type represents possible errors that can occur when downloading a song from deezer

#[derive(Debug, Clone)]
pub enum DeezerApiError {
    /// this error occurs either when downloader token doesn't match with the one in cookies
    InvalidToken,
    /// this error might occur during the decryption process
    DecryptionError,
    /// any other error, minor error on the side of deezer api, such as wrong method, id, etc
    OtherError(String),
}

impl From<BlockModeError> for DeezerApiError {
    fn from(_: BlockModeError) -> Self {
        DeezerApiError::DecryptionError
    }
}

impl From<&Map<String, Value>> for DeezerApiError {
    fn from(errors: &Map<String, Value>) -> Self {
        if errors.contains_key("VALID_TOKEN_REQUIRED") {
            DeezerApiError::InvalidToken
        } else {
            DeezerApiError::OtherError(
                errors.iter().fold("".to_string(), |result, (key, value)| {
                    format!("{}{}:{}\n", result, key, value)
                }),
            )
        }
    }
}

impl std::fmt::Display for DeezerApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            DeezerApiError::DecryptionError => "failed to decrypt file",
            DeezerApiError::InvalidToken => "invalid CSRF token",
            DeezerApiError::OtherError(message) => message.as_str(),
        };
        write!(f, "{}", message)
    }
}

impl Error for DeezerApiError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
