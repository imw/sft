use rustfft::num_complex::Complex;

use rustfft::algorithm::Dft;
use rustfft::{Fft, FftDirection};

/*
int get_psd (double *data, int data_len, int sampling_rate, double *output_ampl, double *output_freq)
{
    double *re = new double[data_len / 2 + 1];
    double *im = new double[data_len / 2 + 1];
    int res = perform_fft (data, data_len, window_function, re, im);

    double freq_res = (double)sampling_rate / (double)data_len;
    for (int i = 0; i < data_len / 2 + 1; i++)
    {
        // https://www.mathworks.com/help/signal/ug/power-spectral-density-estimates-using-fft.html
        output_ampl[i] = (re[i] * re[i] + im[i] * im[i]) / ((double)(sampling_rate * data_len));
        if ((i != 0) && (i != data_len / 2))
        {
            output_ampl[i] *= 2;
        }
        output_freq[i] = i * freq_res;
    }

}
 */

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
    //TODO
    let mut signal: Vec<Complex<f32>> = Vec::new();
    for i in 0..123 {
        let val = Complex{
            re: (i as f32).sin(),
            im: 0.0f32
        };
        signal.push(val);
    }


    //println!{"before process: {:?}", signal};
    let dft = Dft::new(123, FftDirection::Forward);
    dft.process(&mut signal);
    //println!{"{:?}", signal};
    for (k,v) in signal.iter().enumerate() {
        println!{"{},{},{}",k,v.re*10_000_000f32,v.im*10_000_000f32};
    }

    let (output_amplitude, output_freq) = get_psd(&signal, 60f32);
    for i in 0..output_freq.len() {
        println!{"{},{},{}",i, output_amplitude[i], output_freq[i]};
    }

}
