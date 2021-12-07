use block_modes::BlockModeError;
use serde_json::{Map, Value};
use std::error::Error;

#[derive(Debug)]
pub struct BlowfishCypherError(pub BlockModeError);

impl std::fmt::Display for BlowfishCypherError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Couldn't decode a song")
    }
}

impl Error for BlowfishCypherError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[derive(Debug, Clone)]
pub struct DeezerApiError {
    error_message: String,
}

impl DeezerApiError {
    pub fn new(errors: &Map<String, Value>) -> Self {
        let error_message = errors.iter().fold("".to_string(), |result, (key, value)| {
            format!("{}{}:{}\n", result, key, value)
        });
        DeezerApiError { error_message }
    }
}

impl std::fmt::Display for DeezerApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DeezerApi encountered an error: {}!", self.error_message)
    }
}

impl Error for DeezerApiError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
