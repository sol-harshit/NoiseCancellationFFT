use hound;

pub fn save_to_wav(pcm_data: Vec<i16>, output_path: &str) {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create(output_path, spec).unwrap();
    for sample in pcm_data {
        writer.write_sample(sample).unwrap();
    }
    writer.finalize().unwrap();
}