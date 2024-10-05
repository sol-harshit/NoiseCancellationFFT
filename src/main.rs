use crate::decoder::decode_wav_to_pcm;

mod decoder;
mod noise_reduction;
mod encoder;

fn main() {
    let input_mp3_file = "asddasd.mp3";
    //let input_wav_file = "340048.wav";
    let output_file = "output_46836.wav";

    // Step 1: Decode MP3 file to PCM data
    let result_data = decoder::decode_mp3_to_pcm(input_mp3_file);
    // let result_data = decode_wav_to_pcm(input_wav_file);
    let pcm_data = match result_data {
        Ok(data) => {
            if data.is_empty() {
                eprintln!("Decoded PCM data is empty.");
                return;
            }
            println!("Decoded PCM data length: {}", data.len());
            data
        },
        Err(e) => {
            eprintln!("Failed to decode audio file: {}", e);
            return;
        }
    };

    // Step 2: Apply noise reduction to PCM data
    let processed_data = noise_reduction::apply_noise_reduction(pcm_data);
    println!("Noise reduction applied to PCM data");

    // Step 3: Encode the processed PCM data to WAV file
    encoder::save_to_wav(processed_data, output_file);
    println!("Processed audio saved to {}", output_file);

}