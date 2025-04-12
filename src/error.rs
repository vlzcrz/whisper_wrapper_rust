use std::path::PathBuf;
use thiserror::Error;

/// Errors that can occur when using the whisper wrapper
#[derive(Error, Debug)]
pub enum WhisperError {
    /// Error when initializing the whisper context
    #[error("Failed to initialize whisper context: {0}")]
    InitializationError(String),

    /// Error when loading the model
    #[error("Failed to load model from {path}: {message}")]
    ModelLoadError { path: PathBuf, message: String },

    /// Error when transcribing audio
    #[error("Failed to transcribe audio: {0}")]
    TranscriptionError(String),

    /// Error when downloading a model
    #[cfg(feature = "download")]
    #[error("Failed to download model: {0}")]
    DownloadError(String),

    /// Error when the model file is not found
    #[error("Model file not found: {0}")]
    ModelNotFound(PathBuf),

    /// Error when the audio file is not found
    #[error("Audio file not found: {0}")]
    AudioNotFound(PathBuf),

    /// Error when the audio file format is not supported
    #[error("Unsupported audio format: {0}")]
    UnsupportedAudioFormat(String),

    /// IO error
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// Other error
    #[error("{0}")]
    Other(String),
}

#[cfg(feature = "download")]
impl From<reqwest::Error> for WhisperError {
    fn from(err: reqwest::Error) -> Self {
        WhisperError::DownloadError(err.to_string())
    }
}
