use rustfft::FftPlanner;
use rustfft::num_complex::Complex;

pub fn apply_noise_reduction(pcm_data: Vec<i16>) -> Vec<i16> {
    let mut planner =  FftPlanner::new();
    let fft = planner.plan_fft_forward(pcm_data.len());

    //  Convert PCM data to complex numbers for FFT
    let mut complex_input: Vec<Complex<f32>> = pcm_data
        .iter()
        .map(|&sample| Complex::new(sample as f32, 0.0))
        .collect();

    // Perform FFT
    fft.process(&mut complex_input);

    pcm_data

}