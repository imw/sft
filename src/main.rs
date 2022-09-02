use rustfft::num_complex::Complex;
use rustfft::algorithm::Dft;
use rustfft::{Fft, FftDirection};

use sft::dft;


fn get_psd(fft_output: &Vec<Complex<f32>>, sampling_rate: f32) -> (Vec<f32>, Vec<f32>){
    let mut output_amplitude = vec![0.0f32; fft_output.len()];
    let mut output_freq = vec![0.0f32; fft_output.len()];
    let data_len= (fft_output.len() -1)*2;
    let freq_res = sampling_rate / data_len as f32;
    for (i, v) in fft_output.iter().enumerate() {
        output_amplitude[i] = (v.re*v.re + v.im*v.im)/(sampling_rate*data_len as f32);
        if i != 0 && i != data_len/2 {
            output_amplitude[i] *= 2f32;
        }
        output_freq[i] = i as f32 * freq_res;
    }
    return (output_amplitude, output_freq)
}

fn main() {
    let mut signal: Vec<Complex<f32>> = Vec::new();
    for i in 0..123 {
        let val = Complex{
            re: (i as f32).sin(),
            im: 0.0f32
        };
        signal.push(val);
    }

    let cloned_signal = signal.clone();

    //println!{"before process: {:?}", signal};
    let rustfft = Dft::new(123, FftDirection::Forward);
    rustfft.process(&mut signal);

    let dft_in = cloned_signal.clone();
    //dft(signal: Vec<Complex<f32>>) -> Vec<Complex<f32>>
    let dft_out = dft(dft_in);

    //println!{"{:?}", signal};
    // for (k,v) in signal.iter().enumerate() {
    //     println!{"{},{},{}",k,v.re,v.im};
    // }

    let (output_amplitude, output_freq) = get_psd(&signal, 60f32);
    for i in 0..output_freq.len() {
        println!{"{}, {}, {}",signal[i], output_amplitude[i], output_freq[i]};
    }

    let (output_amplitude, output_freq) = get_psd(&dft_out, 60f32);
    for i in 0..output_freq.len() {
        println!{"{}, {}, {}",cloned_signal[i], output_amplitude[i], output_freq[i]};
    }

}
