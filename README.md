# Whisper Wrapper Rust

A Rust wrapper for [whisper.cpp](https://github.com/ggml-org/whisper.cpp), providing an easy-to-use interface for downloading models and transcribing audio files.

## Features

- Download Whisper models directly from HuggingFace
- Transcribe audio files to text
- Support for multiple output formats (txt, srt, vtt, json)
- Language selection for transcription
- Command-line interface

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
whisper_wrapper_rust = "0.1.0"
```

## Requirements

- Rust 1.56 or later
- Git (for cloning whisper.cpp)
- C++ compiler (for building whisper.cpp)
- Make

## Usage

### As a Library

```rust
use whisper_wrapper_rust::{WhisperContext, WhisperParams};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Download a model (if download feature is enabled)
    #[cfg(feature = "download")]
    let model_path = whisper_wrapper_rust::download_model("base")?;

    #[cfg(not(feature = "download"))]
    let model_path = Path::new("path/to/model.bin");

    // Create a whisper context
    let mut ctx = WhisperContext::new(&model_path)?;

    // Set parameters
    let params = WhisperParams::new()
        .language("auto")
        .translate(false)
        .output_format("txt");

    // Transcribe audio
    let result = ctx.transcribe(Path::new("audio.mp3"), &params)?;

    // Print the result
    println!("{}", result);

    Ok(())
}
```

### Command Line Interface

If you enable the `cli` feature, you can use the library as a command-line tool:

#### Download a model

```bash
whisper_cli download --model base
```

Available models: tiny, base, small, medium, large

#### Transcribe an audio file

```bash
whisper_cli transcribe --audio path/to/audio.mp3 --model path/to/model.bin
```

Additional options:

- `--language`: Language code (default: auto)
- `--translate`: Translate to English (flag)
- `--format`: Output format (txt, srt, vtt, json) (default: txt)
- `--output`: Output file path (default: same as input with new extension)

## Features

The library provides several feature flags to customize its functionality:

- `download`: Enable model downloading functionality (requires internet access)
- `cli`: Build the command-line interface
- `default`: Enables both `download` and `cli` features

To use the library without the download functionality:

```toml
[dependencies]
whisper_wrapper_rust = { version = "0.1.0", default-features = false }
```

## How it Works

This wrapper:

1. Downloads and builds the whisper.cpp library during compilation
2. Generates Rust bindings to the C API
3. Provides a safe, idiomatic Rust interface

## License

This project is licensed under the MIT License - see the LICENSE file for details.
