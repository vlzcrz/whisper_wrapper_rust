use anyhow::Result;
use clap::{Parser, Subcommand};
use env_logger;
use log::info;
use std::path::PathBuf;

use whisper_wrapper_rust::{WhisperContext, WhisperParams};

#[cfg(feature = "download")]
use whisper_wrapper_rust::download_model;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Download a Whisper model
    #[cfg(feature = "download")]
    Download {
        /// Model size to download (tiny, base, small, medium, large)
        #[arg(short, long, default_value = "base")]
        model: String,
    },

    /// Transcribe audio to text
    Transcribe {
        /// Path to the audio file
        #[arg(short, long)]
        audio: PathBuf,

        /// Path to the model file
        #[arg(short, long)]
        model: PathBuf,

        /// Language to use for transcription (auto for auto-detect)
        #[arg(short, long, default_value = "auto")]
        language: String,

        /// Whether to translate to English
        #[arg(short, long)]
        translate: bool,

        /// Output format (txt, srt, vtt, json)
        #[arg(short, long, default_value = "txt")]
        format: String,

        /// Output file path (defaults to audio filename with new extension)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}

fn main() -> Result<()> {
    env_logger::init();
    let cli = Cli::parse();

    match &cli.command {
        #[cfg(feature = "download")]
        Commands::Download { model } => {
            info!("Downloading {} model...", model);
            let model_path = download_model(model)?;
            println!("Model downloaded successfully to {:?}!", model_path);
        }

        Commands::Transcribe {
            audio,
            model,
            language,
            translate,
            format,
            output,
        } => {
            info!("Transcribing audio file: {:?}", audio);

            // Create the context
            let mut ctx = WhisperContext::new(model)?;

            // Set parameters
            let params = WhisperParams::new()
                .language(language)
                .translate(*translate)
                .output_format(format);

            // Transcribe
            let result = ctx.transcribe(audio, &params)?;

            // Determine output path
            let output_path = output.clone().unwrap_or_else(|| {
                let mut path = audio.clone();
                path.set_extension(format);
                path
            });

            // Write the result to the output file
            std::fs::write(&output_path, result)?;

            println!("Transcription complete! Output saved to {:?}", output_path);
        }
    }

    Ok(())
}
