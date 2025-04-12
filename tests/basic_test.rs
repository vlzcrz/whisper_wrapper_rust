use std::path::Path;
use whisper_wrapper_rust::{WhisperContext, WhisperParams};

#[test]
#[ignore] // Ignore by default as it requires a model file
fn test_context_creation() {
    // This test requires a model file to be present
    // You can download one using the download_model function
    let model_path = Path::new("path/to/model.bin");

    if !model_path.exists() {
        println!("Skipping test as model file doesn't exist");
        return;
    }

    // Create a context
    let ctx = WhisperContext::new(model_path);
    assert!(ctx.is_ok(), "Failed to create context: {:?}", ctx.err());
}

#[test]
fn test_params_creation() {
    // Test creating parameters
    let params = WhisperParams::new()
        .language("en")
        .translate(true)
        .output_format("txt");

    assert_eq!(params.get_language(), "en");
    assert_eq!(params.get_translate(), true);
    assert_eq!(params.get_output_format(), "txt");
}

#[cfg(feature = "download")]
#[test]
#[ignore] // Ignore by default as it requires internet connection
fn test_download_model() {
    use whisper_wrapper_rust::download_model;

    // This test requires internet connection
    let result = download_model("tiny");
    assert!(
        result.is_ok(),
        "Failed to download model: {:?}",
        result.err()
    );
}
