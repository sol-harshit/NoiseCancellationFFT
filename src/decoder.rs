use symphonia::core::codecs::{DecoderOptions, CODEC_TYPE_MP3};
use symphonia::core::errors::Error;
use symphonia::core::formats::{FormatOptions};
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use symphonia::default::get_probe;
use symphonia::core::audio::{AudioBufferRef, Signal};
use std::fs::File;
use hound::{WavReader};

pub fn decode_mp3_to_pcm(file_path: &str) -> Result<Vec<i16>, Error> {
    // Open the MP3 file directly (no need for BufReader)
    let file = File::open(file_path).expect("Failed to open MP3 file");

    // Create a media source stream directly from the file
    let mss = MediaSourceStream::new(Box::new(file), Default::default());

    // Create a format hint to let Symphonia know it's an MP3 file
    let mut hint = Hint::new();
    hint.with_extension("mp3");

    // Use the default probe to detect the format
    let probed = get_probe().format(
        &hint,
        mss,
        &FormatOptions::default(),
        &MetadataOptions::default(),
    )?;

    // Get the format reader
    let mut format = probed.format;

    // Find the first track that has a valid MP3 codec by matching against the codec type
    let track = format
        .tracks()
        .iter()
        .find(|track| track.codec_params.codec == CODEC_TYPE_MP3)  // Check for MP3 codec type
        .expect("No MP3 audio track found");

    // Create a decoder for the track
    let mut decoder = symphonia::default::get_codecs().make(
        &track.codec_params,
        &DecoderOptions::default(),
    )?;

    // A buffer to hold the decoded PCM samples in i16 format
    let mut pcm_samples = Vec::new();

    // Decode all packets from the file
    while let Ok(packet) = format.next_packet() {
        // Decode the packet into audio samples
        if let Ok(decoded) = decoder.decode(&packet) {
            // Get the audio buffer from the decoder
            match decoded {
                AudioBufferRef::F32(ref buffer) => {
                    // Dereference the Cow to access the AudioBuffer and convert f32 samples to i16
                    for sample in buffer.as_ref().chan(0) {
                        let amplitude = (*sample * i16::MAX as f32) as i16;
                        pcm_samples.push(amplitude);
                    }
                }
                AudioBufferRef::S16(ref buffer) => {
                    // Dereference the Cow to access the AudioBuffer and copy i16 samples directly
                    for sample in buffer.as_ref().chan(0) {
                        pcm_samples.push(*sample);
                    }
                }
                _ => panic!("Unsupported sample format"),
            }
        }
    }

    Ok(pcm_samples)
}

pub fn decode_wav_to_pcm(file_path: &str) -> Result<Vec<i16>, hound::Error> {
    // Open the WAV file using the Hound library
    let reader = WavReader::open(file_path)?;

    // Collect all samples from the WAV file
    let mut pcm_samples = Vec::new();

    // Read all samples from the WAV file and store them in the pcm_samples vector
    for sample in reader.into_samples::<i32>() {
        let sample = sample?;
        // Convert i32 samples to i16
        let sample = (sample >> 16) as i16;
        println!("Sample: {}", sample);
        pcm_samples.push(sample);
    }

    Ok(pcm_samples)
}