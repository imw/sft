use rustfft::num_complex::Complex;

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


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
