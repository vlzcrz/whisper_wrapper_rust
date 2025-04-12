use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let whisper_dir = out_dir.join("whisper.cpp");

    // Clone whisper.cpp if it doesn't exist
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
            panic!("Failed to clone whisper.cpp repository");
        }
    }

    // Build whisper.cpp using CMake
    println!("Building whisper.cpp...");

    // Configure with CMake
    let status = Command::new("cmake")
        .args(&["-B", "build"])
        .current_dir(&whisper_dir)
        .status()
        .expect("Failed to configure whisper.cpp with CMake");

    if !status.success() {
        panic!("Failed to configure whisper.cpp with CMake");
    }

    // Build with CMake
    let status = Command::new("cmake")
        .args(&["--build", "build", "--config", "Release"])
        .current_dir(&whisper_dir)
        .status()
        .expect("Failed to build whisper.cpp with CMake");

    if !status.success() {
        panic!("Failed to build whisper.cpp with CMake");
    }

    // Create wrapper.h
    let wrapper_path = out_dir.join("wrapper.h");

    // Let's list all files in the whisper.cpp directory to find the header
    println!("Searching for whisper.h in the repository...");
    let find_output = Command::new("find")
        .args(&[
            whisper_dir.to_str().unwrap(),
            "-name",
            "whisper.h",
            "-type",
            "f",
        ])
        .output()
        .expect("Failed to execute find command");

    let header_paths = String::from_utf8_lossy(&find_output.stdout)
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| PathBuf::from(s))
        .collect::<Vec<_>>();

    println!("Found whisper.h files: {:?}", header_paths);

    if let Some(header_path) = header_paths.first() {
        println!("Using whisper.h found at: {:?}", header_path);

        fs::write(
            &wrapper_path,
            format!(
                r#"
#include "{}"
                "#,
                header_path.to_str().unwrap()
            ),
        )
        .expect("Failed to write wrapper.h");
    } else {
        // If we can't find it with find, let's create our own header file
        println!("No whisper.h found, creating a custom header");

        fs::write(
            &wrapper_path,
            r#"
// Whisper API
#ifdef __cplusplus
extern "C" {
#endif

#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

// Basic whisper.cpp API declaration
typedef struct whisper_context whisper_context;

whisper_context * whisper_init_from_file(const char * path_model);
void whisper_free(whisper_context * ctx);

#ifdef __cplusplus
}
#endif
            "#,
        )
        .expect("Failed to write custom wrapper.h");
    }

    // Find ggml.h
    println!("Searching for ggml.h in the repository...");
    let ggml_find_output = Command::new("find")
        .args(&[
            whisper_dir.to_str().unwrap(),
            "-name",
            "ggml.h",
            "-type",
            "f",
        ])
        .output()
        .expect("Failed to execute find command for ggml.h");

    let ggml_paths = String::from_utf8_lossy(&ggml_find_output.stdout)
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| PathBuf::from(s))
        .collect::<Vec<_>>();

    println!("Found ggml.h files: {:?}", ggml_paths);

    // Generate bindings
    let mut builder = bindgen::Builder::default()
        .header(wrapper_path.to_str().unwrap())
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()));

    // Add include paths for ggml.h
    if let Some(ggml_path) = ggml_paths.first() {
        let ggml_dir = ggml_path.parent().unwrap();
        println!("Adding include path: {:?}", ggml_dir);
        builder = builder.clang_arg(format!("-I{}", ggml_dir.to_str().unwrap()));
    }

    let bindings = builder.generate().expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file
    let bindings_path = out_dir.join("bindings.rs");
    bindings
        .write_to_file(bindings_path)
        .expect("Couldn't write bindings!");

    // Link with the built library
    println!(
        "cargo:rustc-link-search={}/build/src",
        whisper_dir.to_str().unwrap()
    );
    println!(
        "cargo:rustc-link-search={}/build/ggml/src",
        whisper_dir.to_str().unwrap()
    );
    println!("cargo:rustc-link-lib=whisper");
    println!("cargo:rustc-link-lib=ggml");
    println!("cargo:rustc-link-lib=dylib=stdc++");
}
