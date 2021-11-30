mod sieve_of_eratosthenes;
mod is_even;
mod pseudo_prime;
mod modular_pow;
mod gcd;
mod find_first_d;
mod jacobi_symbol;
mod quantum_register;
mod fourier;
mod log2;
mod lucas;
mod baillie_psw_prime;
mod prime_power_check;
mod plots;
mod logger;
mod continued_fractions;

use sieve_of_eratosthenes::sieve_of_eratosthenes;
use is_even::is_even;
use pseudo_prime::pseudo_prime;
use log::{info, debug, warn};
use modular_pow::modular_pow;

use find_first_d::find_first_d;

use quantum_register::{create_quantum_register, init_quantum_register, print_quantum_register,
                       transform_quantum_register, measure_quantum_register1, measure_quantum_register2};
use crate::baillie_psw_prime::baillie_psw_prime;
use crate::fourier::{create_qft};
use crate::gcd::gcd;
use crate::log2::{log2flt, log2int};
use crate::logger::init_logger;
use crate::lucas::u_k;
use crate::plots::plot_probabilities_register1;
use crate::prime_power_check::prime_power_check;
use crate::quantum_register::{extract_quantum_register1, insert_quantum_register1};
use rustfft::{FftPlanner, num_complex::Complex};

fn main() {
    init_logger();

    // Choose n to factorize
    for n in [974069_u64] {
        println!("n = {}", n);

        // Step 1
        // Only continue if n is not even, not prime and not a power of a prime
        info!("Determine if the number n is a prime, a even number, or an integer power of a prime number (step 1)");
        if !is_even(n) && !baillie_psw_prime(n) && !prime_power_check(n) {
            // Step 2
            // Pick an integer q that is the power of 2 such that n^2 <= q < 2n^2
            info!("Pick an integer q that is the power of 2 such that n^2 <= q < 2n^2 (step 2)");
            let q = 2_u64.pow(((log2flt((2 * n * n) as f64) - 1.0).ceil()) as u32);

            println!("q = {}", q);

            // Step 3
            // Pick a random integer x that is coprime to n.
            for x in [97_u64] {
                info!("Pick a random integer x that is coprime to n (step 3)");
                if gcd(x, n) == 1 {
                    // Select minimum register size
                    // Register 1 must have enough qubits to represent integers as large as q - 1.
                    // Register 2 must have enough qubits to represent integers as large as n - 1.
                    let n_bits_reg1 = log2int(q);
                    let n_bits_reg2 = log2int(n);

                    info!("Register size reg1 = {}, reg2 = {}", n_bits_reg1, n_bits_reg2);

                    info!("Create quantum register in zero state (step 4).");
                    let mut quantum_register = create_quantum_register(n_bits_reg1 as usize, n_bits_reg2 as usize);
                    // print_quantum_register(&quantum_register);

                    info!("Init quantum register for Shor (step 5)");
                    init_quantum_register(&mut quantum_register);

                    print_quantum_register(&quantum_register, true);


                    info!("Apply the transformation x^a mod n to for each number stored in register 1 and store the result in register 2 (step 6)");
                    transform_quantum_register(&mut quantum_register, x, n);
                    print_quantum_register(&quantum_register, true);

                    info!("Measure the second register (step 7)");
                    let reg2_value = measure_quantum_register2(&mut quantum_register);
                    info!("Measured: {}", reg2_value);
                    print_quantum_register(&quantum_register, true);

                    info!("Perform qft on register 1 (step 8)");

                    let mut reg1 = extract_quantum_register1(&mut quantum_register, reg2_value);

                    let mut planner = FftPlanner::new();
                    let fft = planner.plan_fft_forward(reg1.len());
                    fft.process(&mut reg1);

                    // re-normalize (needed for rustfft: https://docs.rs/rustfft/6.0.1/rustfft/)
                    // TODO: check for correctness
                    // TODO: can we vectorize this?
                    let mut total_chance = 0.0;
                    for i in 0..reg1.len() {
                        total_chance += reg1[i].norm() * reg1[i].norm();
                    }

                    let nrm_factor = total_chance.sqrt();

                    for i in 0..reg1.len() {
                        reg1[i] = reg1[i] / nrm_factor;
                    }


                    insert_quantum_register1(&reg1, &mut quantum_register, reg2_value);

                    print_quantum_register(&quantum_register, false);

                    // plot before destroying
                    plot_probabilities_register1(&reg1, format!("n_{}_q_{}_x_{}", n, q, x));

                    info!("Measure register 1 (step 9)");
                    let result = measure_quantum_register1(&mut quantum_register);

                    print_quantum_register(&quantum_register, true);
                    println!("Result: {}", result);
                } else {
                    warn!("x = {}, not coprime to {}, skipped", x, n);
                }
            }
        } else {
            warn!("n= {} even, prime or a prime power, skipped", n);
        }
    }
}
