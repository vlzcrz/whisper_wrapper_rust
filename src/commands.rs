use anyhow::{Context, Result};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Executes the whisper.cpp binary directly with the given arguments
pub fn execute_whisper_cpp(
    model_path: &Path,
    audio_path: &Path,
    output_path: Option<&Path>,
    language: Option<&str>,
    translate: bool,
    output_format: Option<&str>,
    additional_args: Vec<String>,
    binary_path: Option<PathBuf>,
) -> Result<String> {
    // Find the whisper.cpp binary
    let whisper_binary = if let Some(path) = binary_path {
        path
    } else {
        find_whisper_binary()?
    };

    // Build the command
    let mut cmd = Command::new(&whisper_binary);

    // Add the model path
    cmd.arg("-m").arg(model_path);

    // Add the audio path
    cmd.arg(audio_path);

    // Add language if specified
    if let Some(lang) = language {
        if lang != "auto" {
            cmd.arg("-l").arg(lang);
        }
    }

    // Add translate flag if needed
    if translate {
        cmd.arg("--translate");
    }

    // Add output format if specified
    if let Some(format) = output_format {
        match format {
            "txt" => cmd.arg("--output-txt"),
            "srt" => cmd.arg("--output-srt"),
            "vtt" => cmd.arg("--output-vtt"),
            "json" => cmd.arg("--output-json"),
            _ => return Err(anyhow::anyhow!("Unsupported output format: {}", format)),
        };
    }

    // Add output path if specified
    if let Some(path) = output_path {
        cmd.arg("-o").arg(path);
    }

    // Add any additional arguments
    for arg in additional_args {
        cmd.arg(arg);
    }

    // Execute the command
    let output = cmd
        .output()
        .with_context(|| format!("Failed to execute whisper.cpp command"))?;

    // Check if the command was successful
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("whisper.cpp command failed: {}", stderr));
    }

    // Return the stdout as a string
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    Ok(stdout)
}

/// Finds the path to the whisper.cpp binary
pub fn find_whisper_binary() -> Result<PathBuf> {
    // Try common binary names
    let binary_names = vec!["whisper", "main"];

    // Try to find the binary in PATH
    for name in &binary_names {
        let which_cmd = if cfg!(target_os = "windows") {
            "where"
        } else {
            "which"
        };

        let output = Command::new(which_cmd).arg(name).output();

        if let Ok(output) = output {
            if output.status.success() {
                let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
                return Ok(PathBuf::from(path));
            }
        }
    }

    // If not found in PATH, try common locations
    let common_locations = vec![
        "/usr/local/bin/whisper",
        "/usr/bin/whisper",
        "/opt/whisper/bin/whisper",
        "/usr/local/bin/main",
        "/usr/bin/main",
        "/opt/whisper/bin/main",
    ];

    for location in &common_locations {
        let path = PathBuf::from(location);
        if path.exists() {
            return Ok(path);
        }
    }

    // Try to find in the build directory
    if let Ok(out_dir) = env::var("OUT_DIR") {
        let out_dir = PathBuf::from(out_dir);
        let whisper_dir = out_dir.join("whisper.cpp");

        let possible_paths = vec![
            whisper_dir.join("build/bin/main"),
            whisper_dir.join("build/main"),
            whisper_dir.join("build/whisper"),
        ];

        for path in possible_paths {
            if path.exists() {
                return Ok(path);
            }
        }
    }

    // If we still can't find it, suggest building it or using the build_whisper_binary function
    Err(anyhow::anyhow!(
        "Could not find whisper.cpp binary. Please make sure whisper.cpp is built and the binary is in your PATH, \
        or specify the path to the binary using the --binary option, or use the build_whisper_binary function to build it."
    ))
}

/// Builds the whisper.cpp binary and returns the path to the built binary
pub fn build_whisper_binary() -> Result<PathBuf> {
    // Create a temporary directory for building whisper.cpp
    let temp_dir = env::temp_dir().join("whisper_cpp_build");
    if !temp_dir.exists() {
        fs::create_dir_all(&temp_dir)?;
    }

    // Clone whisper.cpp if it doesn't exist
    let whisper_dir = temp_dir.join("whisper.cpp");
    if !whisper_dir.exists() {
        println!("Cloning whisper.cpp repository...");
        let status = Command::new("git")
            .args(&[
                "clone",
                "https://github.com/ggml-org/whisper.cpp",
                whisper_dir.to_str().unwrap(),
            ])
            .status()
            .expect("Failed to clone whisper.cpp repository");

        if !status.success() {
            return Err(anyhow::anyhow!("Failed to clone whisper.cpp repository"));
        }
    }

    // Build whisper.cpp
    println!("Building whisper.cpp...");
    let status = Command::new("make")
        .current_dir(&whisper_dir)
        .status()
        .expect("Failed to build whisper.cpp");

    if !status.success() {
        return Err(anyhow::anyhow!("Failed to build whisper.cpp"));
    }

    // Check if the binary was built successfully
    let binary_path = whisper_dir.join("main");
    if !binary_path.exists() {
        return Err(anyhow::anyhow!(
            "Failed to find the built whisper.cpp binary"
        ));
    }

    println!(
        "whisper.cpp binary built successfully at: {:?}",
        binary_path
    );
    Ok(binary_path)
}
