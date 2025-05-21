mod metal;

use anyhow::{Result};
use pyo3::{prelude::*};
use std::io::{stdin, BufRead};
use std::ffi::CString;
use metal::MetalContext;

fn main() -> Result<()> {
    // Initialize Metal context
    let metal_context = MetalContext::new()?;
    println!("Metal device initialized successfully");

    // Print startup message
    println!("Initializing Python runtime...");

    Python::with_gil(|py| {
        println!("Downloading and loading models...");

        let module = PyModule::from_code(
            py,
            &CString::new(include_str!("../huggingface.py"))?.as_c_str(),
            &CString::new("huggingface.py")?.as_c_str(),
            &CString::new("huggingface")?.as_c_str(),
        )?;

        let text_to_speech: Py<PyAny> = module.getattr("text_to_speech")?.into();

        println!("Done! Type a sentence and hit enter. To exit hold Ctrl+C and hit Enter");

        let stdin = stdin();

        for line in stdin.lock().lines() {
            let Ok(text) = line else {
                break;
            };

            let samples: Vec<f32> = text_to_speech.call1(py, (text,))?.extract(py)?;
            
            // Create Metal buffer for the audio samples
            let buffer = metal_context.create_buffer(&samples);
            println!("Created Metal buffer with {} samples", samples.len());
            
            dbg!(samples.len());
        }

        Ok(())
    })
} 