use rustfft::num_complex::Complex;

use rustfft::algorithm::Dft;
use rustfft::{Fft, FftDirection};

fn main() {
    //TODO
    let mut signal: Vec<Complex<f32>> = Vec::new();
    for i in 0..123 {
        let arity: f32 = (i as f32) % (3 as f32);
        let val = Complex{
            re: 1.0f32 * arity,
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
}
