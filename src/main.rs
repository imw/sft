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

fn dft(signal: Vec<Complex<f32>>) -> Vec<Complex<f32>> {
	let mut output = Vec::new();

	let n: usize = signal.len();
	for k in 0 .. n {  // For each output element

		let mut sumreal: f32 = 0.0;
		let mut sumimag: f32 = 0.0;
		for t in 0 .. n {  // For each input element
			let angle: f32 = 2.0 * std::f32::consts::PI * (t as f32) * (k as f32) / (n as f32);
			sumreal +=  signal[t].re * angle.cos() + signal[t].im * angle.sin();
			sumimag += -signal[t].re * angle.sin() + signal[t].im * angle.cos();
		}
		output.push(Complex{re:sumreal, im:sumimag});
	}
	output
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

    let cloned_input = signal.clone();

    //println!{"before process: {:?}", signal};
    let rustfft = Dft::new(123, FftDirection::Forward);
    rustfft.process(&mut signal);

    let dft_in = cloned_input.clone();
    //dft(signal: Vec<Complex<f32>>) -> Vec<Complex<f32>>
    let dft_out = dft(dft_in);

    //println!{"{:?}", signal};
    // for (k,v) in signal.iter().enumerate() {
    //     println!{"{},{},{}",k,v.re,v.im};
    // }

    let (output_amplitude, output_freq) = get_psd(&signal, 60f32);
    for i in 0..output_freq.len() {
        println!{"{}, {}, {}",cloned_input[i], output_amplitude[i], output_freq[i]};
    }

    let (output_amplitude, output_freq) = get_psd(&dft_out, 60f32);
    for i in 0..output_freq.len() {
        println!{"{}, {}, {}",cloned_input[i], output_amplitude[i], output_freq[i]};
    }

}
