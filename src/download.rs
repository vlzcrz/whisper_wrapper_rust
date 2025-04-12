use anyhow::{anyhow, Result};
use dirs::home_dir;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

#[cfg(feature = "indicatif")]
use indicatif::{ProgressBar, ProgressStyle};

// Model URLs
const MODEL_URLS: &[(&str, &str)] = &[
    (
        "tiny",
        "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-tiny.bin",
    ),
    (
        "base",
        "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.bin",
    ),
    (
        "small",
        "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-small.bin",
    ),
    (
        "medium",
        "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-medium.bin",
    ),
    (
        "large",
        "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-large.bin",
    ),
];

// Get the models directory
fn get_models_dir() -> PathBuf {
    let mut models_dir = home_dir().unwrap_or_else(|| PathBuf::from("."));
    models_dir.push(".whisper-models");

    if !models_dir.exists() {
        fs::create_dir_all(&models_dir).expect("Failed to create models directory");
    }

    models_dir
}

// Download a file with progress bar if indicatif is enabled
fn download_file(url: &str, path: &Path) -> Result<()> {
    println!("Downloading from {}", url);

    let client = reqwest::blocking::Client::new();
    let resp = client.get(url).send()?;
    let total_size = resp.content_length().unwrap_or(0);

    #[cfg(feature = "indicatif")]
    let pb = {
        let pb = ProgressBar::new(total_size);
        pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .unwrap()
            .progress_chars("#>-"));
        pb
    };

    let mut file = File::create(path)?;
    let content = resp.bytes()?;
    file.write_all(&content)?;

    #[cfg(feature = "indicatif")]
    pb.finish_with_message("Download complete");

    #[cfg(not(feature = "indicatif"))]
    println!("Download complete");

    Ok(())
}

/// Download a Whisper model
///
/// # Arguments
///
/// * `model_name` - The name of the model to download (tiny, base, small, medium, large)
///
/// # Returns
///
/// The path to the downloaded model
///
/// # Examples
///
/// ```no_run
/// use whisper_wrapper_rust::download_model;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let model_path = download_model("base")?;
///     println!("Model downloaded to {:?}", model_path);
///     Ok(())
/// }
/// ```
pub fn download_model(model_name: &str) -> Result<PathBuf> {
    let models_dir = get_models_dir();

    let (_, url) = MODEL_URLS
        .iter()
        .find(|(name, _)| *name == model_name)
        .ok_or_else(|| anyhow!("Invalid model name: {}", model_name))?;

    let model_filename = format!("ggml-{}.bin", model_name);
    let model_path = models_dir.join(&model_filename);

    if model_path.exists() {
        println!("Model already exists at {:?}", model_path);
        return Ok(model_path);
    }

    download_file(url, &model_path)?;
    Ok(model_path)
}

/// List available models that can be downloaded
pub fn list_available_models() -> Vec<String> {
    MODEL_URLS
        .iter()
        .map(|(name, _)| name.to_string())
        .collect()
}

/// List downloaded models
pub fn list_downloaded_models() -> Result<Vec<PathBuf>> {
    let models_dir = get_models_dir();
    let entries = fs::read_dir(models_dir)?;

    let models = entries
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.is_file() && path.extension()? == "bin" {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    Ok(models)
}
