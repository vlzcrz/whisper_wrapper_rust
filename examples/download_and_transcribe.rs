use std::error::Error;
use std::path::Path;
use whisper_wrapper_rust::{WhisperContext, WhisperParams};

fn main() -> Result<(), Box<dyn Error>> {
    // Download the base model (requires the "download" feature to be enabled)
    #[cfg(feature = "download")]
    let model_path = whisper_wrapper_rust::download_model("base")?;

    #[cfg(not(feature = "download"))]
    let model_path = {
        println!("Download feature not enabled. Please provide a path to a model file.");
        Path::new("path/to/your/model.bin").to_path_buf()
    };

    println!("Using model at: {:?}", model_path);

    // Create a whisper context
    let mut ctx = WhisperContext::new(&model_path)?;

    // Path to your audio file
    let audio_path = Path::new("path/to/your/audio.mp3");

    // Check if the audio file exists
    if !audio_path.exists() {
        println!("Audio file not found at: {:?}", audio_path);
        println!("Please update the path to point to a valid audio file.");
        return Ok(());
    }

    // Set parameters
    let params = WhisperParams::new()
        .language("es") // Spanish language
        .translate(true) // Translate to English
        .output_format("srt"); // Output in SRT format

    println!("Transcribing audio file: {:?}", audio_path);

    // Transcribe audio
    let result = ctx.transcribe(audio_path, &params)?;

    // Save the result to a file
    let output_path = "transcription.srt";
    std::fs::write(output_path, &result)?;

    println!("Transcription complete!");
    println!("Result saved to: {}", output_path);
    println!("First few lines:");

    // Print the first few lines of the result
    let preview = result.lines().take(10).collect::<Vec<_>>().join("\n");
    println!("{}", preview);

    Ok(())
}
