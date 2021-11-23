const EPSILON: f64 = 0_f64;
const MAX_PRINT: usize = 10;            // max lines to print before hiding results

use ndarray::Array2;
use num::complex::Complex;
use crate::modular_pow::modular_pow;

use rand::Rng;

pub struct QuantumRegister {
    register: Array2::<Complex<f64>>,
    pub n_bits_reg1: usize, // size of register 1, must have enough qubits to represent q - 1
    n_bits_reg2: usize  // size of register 2, must have enough qubits to represent n - 1
}

// step 4
// create statusvector for a register that represents N_BITS_REG1+N_BITS_REG1 qbits
pub fn create_quantum_register(n_bits_reg1: usize, n_bits_reg2: usize) -> QuantumRegister {
    let mut register = QuantumRegister {
        register: Array2::<Complex<f64>>::zeros((2_usize.pow((n_bits_reg1 + n_bits_reg2) as u32), 1)),
        n_bits_reg1: n_bits_reg1,
        n_bits_reg2: n_bits_reg2
    };

    // set register to a valid state (zero)
    register.register[[0, 0]] = num::complex::Complex::new(1.0, 0.0);
    register
}

// step 5
// load register 1 with an equally weighted superposition of all integers from 0 to q - 1
// load register 2 with all zeros.
pub fn init_quantum_register(register: &mut QuantumRegister) {
    let n_state_vectors_reg1 = 2_u32.pow(register.n_bits_reg1 as u32);
    for i in 0..n_state_vectors_reg1 {
        register.register[[(i as usize) << register.n_bits_reg2, 0]] = num::complex::Complex::new(1./(n_state_vectors_reg1 as f64).sqrt(), 0.0);
    }
}

// step 6
// simulate the quantum transformation x^a mod n to for each number stored in register 1
// and store the result in register 2.
pub fn transform_quantum_register(register: &mut QuantumRegister, x:u64, n:u64) {
    let n_state_vectors_reg1 = 2_u32.pow(register.n_bits_reg1 as u32);
    for a in 0..n_state_vectors_reg1 {
        // perform computation
        let result = modular_pow(x, a as u64, n);
        // combine input and result
        let input_register = (a as usize) << register.n_bits_reg2;
        let output_register = input_register | result as usize;
        // move value
        let input_register_value = register.register[[input_register, 0]];
        register.register[[input_register, 0]] = num::complex::Complex::new(0.0, 0.0);
        register.register[[output_register, 0]] = input_register_value;
    }
}

// show values for all base vectors of the quantum register, and split them into reg1 and reg2
pub fn print_quantum_register(register: &QuantumRegister) {
    let mut print_results = Vec::new();
    println!("reg1 - reg2 (value)");
    // format results
    for i in 0..register.register.len() {
        // only show register values of base vectors with a minimum (square root) probability
        if register.register[[i, 0]].norm() > EPSILON {
            // split base vector to show reg1 and reg2 separately
            // println!("{} - {} ({})", i >> N_BITS_REG2, i as u32 & (2_u32.pow(N_BITS_REG2 as u32) - 1), register[[i, 0]]);
            print_results.push(format!("{} - {} ({})", i >> register.n_bits_reg2, i as u32 & (2_u32.pow(register.n_bits_reg2 as u32) - 1), register.register[[i, 0]]))
        }
    }
    // print results
    let mut dots_printed = false;
    for i in 0..print_results.len() {
        if (i < MAX_PRINT) || (i > print_results.len() - MAX_PRINT - 1) {
            println!("{}", print_results[i]);
        } else {
            if !dots_printed {
                println!("...");
                dots_printed = true;
            }
        }
    }
}


// meassure (a part of) the quantum register
// from_bit: first bit to be measured (0 based, count from left, MSB)
// to_bit: last bit to be measured (same as first bit if you want to measure only one bit, LSB)
pub fn measure_quantum_register(register: &mut QuantumRegister, from_bit:usize, to_bit:usize) -> usize {
    let lsb = register.n_bits_reg1 + register.n_bits_reg2 - to_bit - 1;

    // draw random number [0,1) to select a base vector from the register
    let mut rng = rand::thread_rng();
    let random_chance = rng.gen::<f64>();

    // find randomly selected base vector, using chances stored in the quantum register
    let mut total_chance = 0.0;
    let mut selected_vector:usize = 0; // first vector will be selected if rounding errors prevent a selection
    for i in 0..register.register.len() {
        let chance = register.register[[i, 0]].norm() * register.register[[i, 0]].norm(); // chance = square of magnitude
        total_chance += chance;

        // randomly selected vector found => break
        if total_chance > random_chance {
            selected_vector = i;
            println!("Selected vector for measurement {} - {} ({})", i >> register.n_bits_reg2, i as u32 & (2_u32.pow(register.n_bits_reg2 as u32) - 1), register.register[[i, 0]]);
            break;
        }
    }

    // create mask with ones msb-lsb, and zeros otherwise 111000
    let measured_bits_mask = (2_u32.pow((to_bit - from_bit + 1) as u32) - 1) << lsb;
    // mask selected vector 101101 => 101000
    let masked_selected_vector = selected_vector & (measured_bits_mask as usize);
    // compare masked selected vector to all (masked) other vectors
    // only keep matching vectors, reset others to zero
    let mut remaining_chance = 0.0; // sum up remaining chances to normalize quantum vector
    for i in 0..register.register.len() {
        if i & (measured_bits_mask as usize) != masked_selected_vector {
            // no match, so set to zero
            register.register[[i, 0]] = num::complex::Complex::new(0.0, 0.0);
        } else {
            // vector is a match, keep it and sum up chance total for re-normalization
            remaining_chance += register.register[[i, 0]].norm() * register.register[[i, 0]].norm();
        }
    }

    // normalize the quantum register (chances must sum up to 1)
    for i in 0..register.register.len() {
        register.register[[i, 0]] = register.register[[i, 0]] / remaining_chance.sqrt();
    }

    (selected_vector & (measured_bits_mask as usize)) >> lsb
}

// helper function to measure register 1
pub fn measure_quantum_register1(register: &mut QuantumRegister) -> usize {
    measure_quantum_register(register, 0, register.n_bits_reg1 - 1)
}

// helper function measure register 2
pub fn measure_quantum_register2(register: &mut QuantumRegister) -> usize {
    measure_quantum_register(register, register.n_bits_reg1, register.n_bits_reg1 + register.n_bits_reg2 - 1)
}


// extract vector representation of a register 1, under the assumption
// register 2 has been measured
pub fn extract_quantum_register1(register: &mut QuantumRegister) -> Vec<Complex<f64>> {
    let mut partial_register = vec![Complex{ re: 0.0f64, im: 0.0f64 }; 2_u32.pow(register.n_bits_reg1 as u32) as usize];

    // register 2 has already been measured, measure again to know what reg 2 is to
    // target correct location of reg 1 values
    let reg2_value = measure_quantum_register2(register);

    for i in 0..2_u32.pow(register.n_bits_reg1 as u32) {
        partial_register[i as usize] =  register.register[[(i << register.n_bits_reg2 | (reg2_value as u32)) as usize, 0]];
    }

    // return result (reg1)
    partial_register
}

// insert vector representation of register 1 into full register, under the assumption
// register 2 has been measured
pub fn insert_quantum_register1(partial_register: &Vec<Complex<f64>>, register: &mut QuantumRegister) {
    // register 2 has already been measured, measure again to know what reg 2 is to
    // target correct location of reg 1 values
    let reg2_value = measure_quantum_register2(register);

    for i in 0..2_u32.pow(register.n_bits_reg1 as u32) {
        register.register[[(i << register.n_bits_reg2 | (reg2_value as u32)) as usize, 0]] = partial_register[i as usize];
    }
}
