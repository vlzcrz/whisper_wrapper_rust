use std::ffi::CString;
use std::fs;
use std::path::{Path, PathBuf};

use crate::bindings;
use crate::error::WhisperError;
use crate::params::WhisperParams;
use crate::Result;

/// A context for the Whisper model
pub struct WhisperContext {
    ctx: *mut bindings::whisper_context,
    model_path: PathBuf,
}

impl WhisperContext {
    /// Create a new whisper context from a model file
    pub fn new(model_path: &Path) -> Result<Self> {
        if !model_path.exists() {
            return Err(WhisperError::ModelNotFound(model_path.to_path_buf()));
        }

        let model_path_cstring = CString::new(model_path.to_string_lossy().as_bytes())
            .map_err(|_| WhisperError::InitializationError("Invalid model path".to_string()))?;

        let ctx = unsafe { bindings::whisper_init_from_file(model_path_cstring.as_ptr()) };

        if ctx.is_null() {
            return Err(WhisperError::ModelLoadError {
                path: model_path.to_path_buf(),
                message: "Failed to initialize model".to_string(),
            });
        }

        Ok(Self {
            ctx,
            model_path: model_path.to_path_buf(),
        })
    }

    /// Get the model path
    pub fn model_path(&self) -> &Path {
        &self.model_path
    }

    /// Transcribe an audio file
    pub fn transcribe(&mut self, audio_path: &Path, _params: &WhisperParams) -> Result<String> {
        if !audio_path.exists() {
            return Err(WhisperError::AudioNotFound(audio_path.to_path_buf()));
        }

        // For simplicity, we'll assume the audio is already in the correct format
        // In a real implementation, you'd want to handle different audio formats and convert if necessary
        let _audio_data = fs::read(audio_path).map_err(|e| WhisperError::IoError(e))?;

        // Process the audio data
        // This is a simplified example - in a real implementation, you'd need to:
        // 1. Load the audio file (e.g., using a crate like rodio or hound)
        // 2. Convert it to the format expected by whisper (16kHz mono f32 samples)
        // 3. Pass it to whisper_full

        // For now, we'll just return a placeholder
        let result = format!(
            "Transcription of {:?} using model {:?}",
            audio_path, self.model_path
        );

        // In a real implementation, you'd do something like:
        /*
        let whisper_params = params.to_whisper_params(self.ctx);

        let status = unsafe {
            bindings::whisper_full(
                self.ctx,
                whisper_params,
                audio_samples.as_ptr(),
                audio_samples.len() as i32,
            )
        };

        if status != 0 {
            return Err(WhisperError::TranscriptionError(format!("whisper_full returned {}", status)));
        }

        // Extract the text from the segments
        let n_segments = unsafe { bindings::whisper_full_n_segments(self.ctx) };
        let mut result = String::new();

        for i in 0..n_segments {
            let text = unsafe {
                let text_ptr = bindings::whisper_full_get_segment_text(self.ctx, i);
                CStr::from_ptr(text_ptr).to_string_lossy().into_owned()
            };
            result.push_str(&text);
            result.push('\n');
        }
        */

        Ok(result)
    }
}

impl Drop for WhisperContext {
    fn drop(&mut self) {
        if !self.ctx.is_null() {
            unsafe {
                bindings::whisper_free(self.ctx);
            }
        }
    }
}

// Ensure the context is Send and Sync
unsafe impl Send for WhisperContext {}
unsafe impl Sync for WhisperContext {}
