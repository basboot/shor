mod sieve_of_eratosthenes;
mod is_even;
mod pseudo_prime;
mod modular_pow;
mod gcd;
mod find_first_d;
mod jacobi_symbol;
mod quantum_register;

use sieve_of_eratosthenes::sieve_of_eratosthenes;
use is_even::is_even;
use pseudo_prime::pseudo_prime;

use find_first_d::find_first_d;
use jacobi_symbol::jacobi_symbol;

use quantum_register::{create_quantum_register, print_quantum_register};


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

    println!("Create quantum register (step 4) in zero state");
    let mut quantum_register = create_quantum_register();
    print_quantum_register(&quantum_register);
}
