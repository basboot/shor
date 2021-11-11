
const N_BITS_REG1:usize = 3; // size of register 1, must have enough qubits to represent q - 1
const N_BITS_REG2:usize = 3; // size of register 2, must have enough qubits to represent n - 1
const EPSILON: f64 = 0_f64;

use ndarray::Array2;
use num::complex::Complex;

// create statusvector for a register that represents N_BITS_REG1+N_BITS_REG1 qbits
pub fn create_quantum_register() -> Array2::<Complex<f64>> {
    // TODO: use complex numbers
    let mut register = Array2::<Complex<f64>>::zeros((2_usize.pow((N_BITS_REG1 + N_BITS_REG2) as u32), 1));

    // set register to a valid state (zero)
    register[[0, 0]] = num::complex::Complex::new(1.0, 0.0);
    register
}

pub fn init_quantum_register(register: &mut Array2::<Complex<f64>>) {
    let n_state_vectors_reg1 = 2_u32.pow(N_BITS_REG1 as u32);
    for i in 0..n_state_vectors_reg1 {
        register[[(i as usize) << N_BITS_REG2, 0]] = num::complex::Complex::new(1./(n_state_vectors_reg1 as f64).sqrt(), 0.0);
    }
}

pub fn print_quantum_register(register: &Array2::<Complex<f64>>) {
    println!("reg1 - reg2 (value)");
    for i in 0..register.len() {
        if register[[i, 0]].norm() > EPSILON {
            println!("{} - {} ({})", i >> N_BITS_REG2, i as u32 & (2_u32.pow(N_BITS_REG2 as u32) - 1), register[[i, 0]]);
        }
    }
}