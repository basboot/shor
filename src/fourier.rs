use ndarray::Array2;
use num::Complex;
use std::f64::consts::PI;

fn create_omegas(n: u32) -> Array2::<Complex<f64>> {
    let mut omegas = Array2::<Complex<f64>>::zeros((n as usize, 1));

    let omega = Complex::new(0.0, 2.0*PI/(n as f64)).exp();

    println!("omega = {}", omega);

    // omega^0 = 1
    omegas[[0, 0]] = Complex::new(1.0, 0.0);

    // create other powers
    for i in 1..n {
        omegas[[i as usize, 0]] = omegas[[(i-1) as usize, 0]] * omega;
    }

    omegas
}

pub fn create_qft(n: u32) -> Array2::<Complex<f64>> {
    let omegas = create_omegas(n);

    let mut qft = Array2::<Complex<f64>>::zeros((n as usize, n as usize));

    for i in 0..n {
        for j in 0..n {
            qft[[i as usize, j as usize]] = omegas[[((i*j) % n) as usize,0]]
        }
    }

    println!("1/sqrt(n) * {}", qft);

    qft * (1.0/(n as f64).sqrt())
}
