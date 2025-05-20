# PyO3 ML Coding with HuggingFace

This guide provides helpful information for working with PyO3 to integrate HuggingFace models with Rust applications.

## Project Structure

- `src/main.rs`: Rust application that interfaces with Python/HuggingFace
- `huggingface.py`: Python module containing HuggingFace models
- `requirements.txt`: Python dependencies

## Common Commands

### Environment Setup

```bash
# Create and activate Python virtual environment
python -m venv .venv
source .venv/bin/activate

# Install Python dependencies
pip install -r requirements.txt
```

### Building

```bash
# Build project in debug mode
cargo build

# Build for release
cargo build --release
```

### Running

```bash
# Run the application
cargo run
```

## Debugging Tips

### PyO3 Configuration

- When you see errors about Python integration, check that:
  1. The Python virtual environment is activated
  2. The correct Python packages are installed
  3. PyO3 feature flags are correctly set in Cargo.toml

### Common Errors

1. **"use of undeclared crate or module `pyo3_build_config`"**
   - This occurs when trying to use build dependencies at runtime
   - Solution: Remove runtime calls to build dependencies
   
2. **Python import errors**
   - Check if all Python packages are installed
   - Ensure Python path is correctly set

3. **Linking errors on macOS**
   - May need to uncomment the target-specific rustflags in Cargo.toml:
   ```toml
   [target.x86_64-apple-darwin]
   rustflags = [
     "-C", "link-arg=-undefined",
     "-C", "link-arg=dynamic_lookup",
   ]
   
   [target.aarch64-apple-darwin]
   rustflags = [
     "-C", "link-arg=-undefined",
     "-C", "link-arg=dynamic_lookup",
   ]
   ```

## HuggingFace Integration Tips

1. Model downloading happens on first run, which may take time
2. Large models require significant memory
3. Consider adding progress indicators for long-running operations
4. For audio playback, initialize the audio backend after model loading

## Performance Tips

1. Use PyO3's GIL management to optimize Python interactions
2. Process data in batches where possible
3. Consider moving intensive operations to Rust
4. Cache model outputs when appropriate

## Candle Configuration and Tips

Candle is HuggingFace's ML framework for Rust that enables native ML without Python dependencies.

### Setup

```toml
# Add to Cargo.toml
[dependencies]
candle-core = "0.3.1"
candle-nn = "0.3.1"
candle-transformers = "0.3.1"

# Optional GPU support
candle-cuda = { version = "0.3.1", optional = true }
candle-metal = { version = "0.3.1", optional = true }

[features]
cuda = ["dep:candle-cuda"]
metal = ["dep:candle-metal"]
```

### GPU Selection

```rust
// Select device (CPU, CUDA, or Metal)
let device = match (cuda, metal) {
    (true, _) => Device::Cuda(0),  // Use first CUDA device
    (_, true) => Device::Metal(0), // Use first Metal device
    _ => Device::Cpu,
};
```

### Common Errors

1. **CUDA/Metal backend errors**:
   - Check hardware compatibility
   - Verify correct feature flags (`--features cuda` or `--features metal`)
   - Install required system drivers

2. **Model loading errors**:
   - Ensure model files have correct permissions
   - Check for sufficient disk space
   - Verify safetensors version compatibility

3. **Memory issues**:
   - Use `model.half()` to use fp16 precision for memory savings
   - Implement gradient checkpointing for large models
   - Consider model quantization

### Debugging Strategy

1. Enable Candle logging:
   ```rust
   std::env::set_var("RUST_LOG", "candle=debug");
   env_logger::init();
   ```

2. Profile memory usage:
   ```rust
   use candle_core::utils::cuda_is_available;
   println!("CUDA available: {}", cuda_is_available());
   // For GPU memory tracking
   Device::Cuda(0).cuda_device_sync()?;
   ```

## Learning Resources

- [PyO3 Documentation](https://pyo3.rs/)
- [HuggingFace Transformers](https://huggingface.co/docs/transformers/index)
- [Speech T5 Model](https://huggingface.co/microsoft/speecht5_tts)
- [Candle Documentation](https://github.com/huggingface/candle)
- [Candle Examples](https://github.com/huggingface/candle/tree/main/candle-examples)