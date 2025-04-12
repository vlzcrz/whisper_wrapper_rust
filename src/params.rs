use crate::bindings;
use std::collections::HashMap;

/// Parameters for whisper transcription
pub struct WhisperParams {
    /// The language to use for transcription (auto for auto-detect)
    language: String,

    /// Whether to translate the audio to English
    translate: bool,

    /// The output format (txt, srt, vtt, json)
    output_format: String,

    /// Additional parameters
    extra_params: HashMap<String, String>,
}

impl Default for WhisperParams {
    fn default() -> Self {
        Self {
            language: "auto".to_string(),
            translate: false,
            output_format: "txt".to_string(),
            extra_params: HashMap::new(),
        }
    }
}

impl WhisperParams {
    /// Create a new set of parameters with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the language for transcription
    pub fn language(mut self, language: &str) -> Self {
        self.language = language.to_string();
        self
    }

    /// Set whether to translate the audio to English
    pub fn translate(mut self, translate: bool) -> Self {
        self.translate = translate;
        self
    }

    /// Set the output format
    pub fn output_format(mut self, format: &str) -> Self {
        self.output_format = format.to_string();
        self
    }

    /// Set an additional parameter
    pub fn param(mut self, key: &str, value: &str) -> Self {
        self.extra_params.insert(key.to_string(), value.to_string());
        self
    }

    /// Get the language
    pub fn get_language(&self) -> &str {
        &self.language
    }

    /// Get whether to translate
    pub fn get_translate(&self) -> bool {
        self.translate
    }

    /// Get the output format
    pub fn get_output_format(&self) -> &str {
        &self.output_format
    }

    /// Get an extra parameter
    pub fn get_param(&self, key: &str) -> Option<&String> {
        self.extra_params.get(key)
    }

    /// Convert to whisper_full_params
    pub(crate) fn to_whisper_params(
        &self,
        _ctx: *mut bindings::whisper_context,
    ) -> bindings::whisper_full_params {
        // Get the default parameters using the greedy sampling strategy (0)
        let mut params = unsafe {
            // Use 0 as the sampling strategy (greedy)
            bindings::whisper_full_default_params(0)
        };

        // Set language if not auto
        if self.language != "auto" {
            params.language = self.language.as_ptr() as *const i8;
        }

        // Set translate
        params.translate = self.translate;

        // Set other parameters from extra_params if needed
        // This would require mapping string keys to the appropriate fields in whisper_full_params

        params
    }
}
