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

use sieve_of_eratosthenes::sieve_of_eratosthenes;
use is_even::is_even;
use pseudo_prime::pseudo_prime;

use find_first_d::find_first_d;
use jacobi_symbol::jacobi_symbol;

use quantum_register::{create_quantum_register, init_quantum_register, print_quantum_register,
                       transform_quantum_register, measure_quantum_register1, measure_quantum_register2};
use crate::fourier::{create_qft};
use crate::gcd::gcd;
use crate::log2::{log2flt, log2int};
use crate::lucas::U;
use crate::quantum_register::{extract_quantum_register1, insert_quantum_register1};


fn main() {
    println!("Hello, Shor!");

    let candidate: u64 = 27644437;

    if !is_even(candidate) {
        println!("{} is not even (possible candidate)", candidate);
    } else {
        println!("{} is even (so not a possible candidate)", candidate);
    }

    let primes = sieve_of_eratosthenes();

    if !primes.contains(&(candidate as usize)) {
        println!("{} is not prime (possible candidate)", candidate);
    } else {
        println!("{} is prime (so not a possible candidate)", candidate);
    }

    if !pseudo_prime(candidate) {
        println!("{} is not pseudoprime (possible candidate)", candidate);
    } else {
        println!("{} is pseudoprime (so not a possible candidate)", candidate);
    }

    // Jacobi
    println!("{}", jacobi_symbol(1027, 541));
    println!("{}", jacobi_symbol(1225, 541));
    println!("{}", jacobi_symbol(1082, 541));

    println!("{}", find_first_d(221));
    println!("{}", find_first_d(223));
    println!("{}", find_first_d(229));

    println!();
    println!();

    // Lucas
    println!("Lucas: U(20, 19, 3, -1) = {}", U(20, 19, 3, -1));

    // Log2
    println!("Log2(256) = {}, log2(sqrt(8)) = {}", log2int(256), log2flt(8_f64.sqrt()));

    // gcd
    println!("GCD 17,37 = {}, GCD 768, 2048 = {}", gcd(17, 37), gcd(768, 2048));

    println!();

    println!("Create quantum register (step 4) in zero state");
    let mut quantum_register = create_quantum_register();
    print_quantum_register(&quantum_register);

    println!();

    println!("Init quantum register for Shor (step 5)");
    init_quantum_register(&mut quantum_register);

    print_quantum_register(&quantum_register);

    println!();

    println!("Apply the transformation x^a mod n to for each number stored in register 1 and store the result in register 2 (step 6)");
    transform_quantum_register(&mut quantum_register, 3, 16);
    print_quantum_register(&quantum_register);

    println!();

    println!("Measure the second register (step 7)");
    println!("Measured: {}", measure_quantum_register2(&mut quantum_register));
    print_quantum_register(&quantum_register);

    println!("Perform qft on register 1 (step 8)");
    let qft = create_qft(2_u32.pow(quantum_register::N_BITS_REG1 as u32));
    let mut reg1 = extract_quantum_register1(&mut quantum_register);
    reg1 = qft.dot(&reg1);

    insert_quantum_register1(&reg1, &mut quantum_register);

    print_quantum_register(&quantum_register);

    println!("Measure register 1 (step 9)");
    let result = measure_quantum_register1(&mut quantum_register);

    print_quantum_register(&quantum_register);
    println!("Result: {}", result);

}
