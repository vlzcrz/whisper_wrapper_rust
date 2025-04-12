use std::error::Error;
use std::path::Path;
use whisper_wrapper_rust::{WhisperContext, WhisperParams};

fn main() -> Result<(), Box<dyn Error>> {
    // Path to your model file
    // If you have the download feature enabled, you can download it:
    // let model_path = whisper_wrapper_rust::download_model("base")?;
    let model_path = Path::new("path/to/your/model.bin");

    // Create a whisper context
    let mut ctx = WhisperContext::new(model_path)?;

    // Path to your audio file
    let audio_path = Path::new("path/to/your/audio.mp3");

    // Set parameters
    let params = WhisperParams::new()
        .language("auto")
        .translate(false)
        .output_format("txt");

    // Transcribe audio
    let result = ctx.transcribe(audio_path, &params)?;

    // Print the result
    println!("Transcription result:");
    println!("{}", result);

    Ok(())
}
