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

use sieve_of_eratosthenes::sieve_of_eratosthenes;
use is_even::is_even;
use pseudo_prime::pseudo_prime;

use find_first_d::find_first_d;

use quantum_register::{create_quantum_register, init_quantum_register, print_quantum_register,
                       transform_quantum_register, measure_quantum_register1, measure_quantum_register2};
use crate::baillie_psw_prime::baillie_psw_prime;
use crate::fourier::{create_qft};
use crate::gcd::gcd;
use crate::log2::{log2flt, log2int};
use crate::lucas::u_k;
use crate::plots::plot_probabilities_register1;
use crate::prime_power_check::prime_power_check;
use crate::quantum_register::{extract_quantum_register1, insert_quantum_register1};


fn main() {
    println!("Hello, Shor!");

    // Choose n to factorize
    for n in [15_u64, 35_u64, 69_u64] {
        println!("n = {}", n);

        // Step 1
        // Only continue if n is not even, not prime and not a power of a prime
        if !is_even(n) && !baillie_psw_prime(n) && !prime_power_check(n) {
            // Step 2
            // Pick a integer q that is the power of 2 such that n^2 <= q < 2n^2
            let q = 2_u64.pow(((log2flt((2 * n * n) as f64) - 1.0).ceil()) as u32);

            println!("q = {}", q);

            // Step 3
            // Pick a random integer x that is coprime to n.
            for x in [2_u64, 3_u64, 5_u64, 7_u64, 11_u64, 13_u64] {
                if gcd(x, n) == 1 {
                    // Select minimum register size
                    // Register 1 must have enough qubits to represent integers as large as q - 1.
                    // Register 2 must have enough qubits to represent integers as large as n - 1.
                    let n_bits_reg1 = log2int(q);
                    let n_bits_reg2 = log2int(n);

                    println!("Register size reg1 = {}, reg2 = {}", n_bits_reg1, n_bits_reg2);

                    println!("Create quantum register (step 4) in zero state");
                    let mut quantum_register = create_quantum_register(n_bits_reg1 as usize, n_bits_reg2 as usize);
                    print_quantum_register(&quantum_register);

                    println!();

                    println!("Init quantum register for Shor (step 5)");
                    init_quantum_register(&mut quantum_register);

                    print_quantum_register(&quantum_register);

                    println!();

                    println!("Apply the transformation x^a mod n to for each number stored in register 1 and store the result in register 2 (step 6)");
                    transform_quantum_register(&mut quantum_register, x, n);
                    print_quantum_register(&quantum_register);

                    println!();

                    println!("Measure the second register (step 7)");
                    println!("Measured: {}", measure_quantum_register2(&mut quantum_register));
                    print_quantum_register(&quantum_register);

                    println!("Perform qft on register 1 (step 8)");
                    let qft = create_qft(2_u32.pow(quantum_register.n_bits_reg1 as u32));
                    let mut reg1 = extract_quantum_register1(&mut quantum_register);
                    reg1 = qft.dot(&reg1);

                    insert_quantum_register1(&reg1, &mut quantum_register);

                    print_quantum_register(&quantum_register);

                    // plot before destroying
                    plot_probabilities_register1(&reg1, format!("n_{}_q_{}_x_{}", n, q, x));

                    println!("Measure register 1 (step 9)");
                    let result = measure_quantum_register1(&mut quantum_register);

                    print_quantum_register(&quantum_register);
                    println!("Result: {}", result);
                } else {
                    println!("x = {}, not coprime to {}, skipped", x, n);
                }
            }
        } else {
            println!("n= {} even, prime or a prime power, skipped", n);
        }
    }
}
