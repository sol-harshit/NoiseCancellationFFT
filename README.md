# Noise Cancellation

This project provides a noise cancellation tool for audio files. It supports both MP3 and WAV file formats.

## Features

- Decode MP3 and WAV files to PCM data
- Apply noise reduction to PCM data
- Encode processed PCM data back to WAV files

## Dependencies

- Rust
- Cargo
- `rustfft` for FFT operations
- `hound` for WAV file handling
- `symphonia` for MP3 file handling

## Installation

1. Ensure you have Rust and Cargo installed. If not, you can install them from [here](https://www.rust-lang.org/tools/install).
2. Clone the repository:
    ```sh
    git clone https://github.com/yourusername/noisecancellation.git
    cd noisecancellation
    ```
3. Build the project:
    ```sh
    cargo build --release
    ```

## Usage

### Using a WAV file

To use a WAV file as input, modify the `main.rs` file to uncomment the WAV file section and comment out the MP3 file section:

```rust
fn main() {
    // let input_mp3_file = "asddasd.mp3";
    let input_wav_file = "340048.wav";
    let output_file = "output_46836.wav";

    // Step 1: Decode WAV file to PCM data
    let result_data = decoder::decode_wav_to_pcm(input_wav_file);
    // let result_data = decoder::decode_mp3_to_pcm(input_mp3_file);
    // ...
}
```

