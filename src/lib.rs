//! # Whisper Wrapper Rust
//!
//! A Rust wrapper for [whisper.cpp](https://github.com/ggml-org/whisper.cpp), providing an easy-to-use interface
//! for downloading models and transcribing audio files.
//!
//! ## Features
//!
//! - Download Whisper models directly from HuggingFace
//! - Transcribe audio files to text
//! - Support for multiple output formats (txt, srt, vtt, json)
//! - Language selection for transcription
//!
//! ## Usage
//!
//! ```rust,no_run
//! use whisper_wrapper_rust::{WhisperContext, WhisperParams};
//! use std::path::Path;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Download a model (if download feature is enabled)
//!     #[cfg(feature = "download")]
//!     let model_path = whisper_wrapper_rust::download_model("base")?;
//!     
//!     #[cfg(not(feature = "download"))]
//!     let model_path = Path::new("path/to/model.bin");
//!     
//!     // Create a whisper context
//!     let mut ctx = WhisperContext::new(&model_path)?;
//!     
//!     // Set parameters
//!     let params = WhisperParams::new()
//!         .language("auto")
//!         .translate(false)
//!         .output_format("txt");
//!     
//!     // Transcribe audio
//!     let result = ctx.transcribe(Path::new("audio.mp3"), &params)?;
//!     
//!     // Print the result
//!     println!("{}", result);
//!     
//!     Ok(())
//! }
//! ```

mod bindings;
pub mod commands;
mod context;
mod error;
mod params;

#[cfg(feature = "download")]
mod download;

pub use commands::execute_whisper_cpp;
pub use context::WhisperContext;
pub use error::WhisperError;
pub use params::WhisperParams;

#[cfg(feature = "download")]
pub use download::download_model;

// Re-export Result type
pub type Result<T> = std::result::Result<T, WhisperError>;
