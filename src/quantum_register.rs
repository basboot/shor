
const N_BITS_REG1:usize = 3; // size of register 1, must have enough qubits to represent q - 1
const N_BITS_REG2:usize = 3; // size of register 2, must have enough qubits to represent n - 1

use ndarray::Array2;

// create statusvector for a register that represents N_BITS_REG1+N_BITS_REG1 qbits
pub fn create_quantum_register() -> Array2::<f64> {
    // TODO: use complex numbers
    let mut register = Array2::<f64>::zeros((2_usize.pow((N_BITS_REG1 + N_BITS_REG2) as u32), 1));

    // set register to a valid state (zero)
    register[[0, 0]] = 1.;

    register
}

