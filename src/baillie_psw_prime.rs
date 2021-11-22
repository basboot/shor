// check if an number is a (probable) prime, using the Baillie–PSW primality test
// https://en.wikipedia.org/wiki/Baillie–PSW_primality_test


use crate::{find_first_d, pseudo_prime, sieve_of_eratosthenes, u_k};

pub fn baillie_psw_prime(n: u64) -> bool {
    // Step 1
    // Optionally, perform trial division to check if n is divisible by a small prime number
    // less than some convenient limit.

    let primes = sieve_of_eratosthenes();

    if primes.contains(&(n as usize)) {
        // number is prime
        println!("{} is prime (sieve)", n);
        return true;
    }

    // number could be prime (if it is larger than the upper limit of sieve_of_eratosthenes),
    // so continue

    // Step 2
    // Perform a base 2 strong probable prime test. If n is not a strong probable prime base 2,
    // then n is composite; quit.

    if !pseudo_prime(n) {
        println!("{} is not prime (fermat)", n);
        return false;
    }

    // number is prime or a strong fermat pseudo prime
    // continue

    // Step 3
    // Find the first D in the sequence 5, −7, 9, −11, 13, −15, ...
    // for which the Jacobi symbol (D/n) is −1. Set P = 1 and Q = (1 − D) / 4.

    let d = find_first_d(n as i64);
    let p = 1;
    let q = (1-d) / 4;
    
    let result = u_k(n+1, (n) as i64, p, q); // TODO: klopt k?

    if result % (n as i64) == 0 {
        println!("{} is prime (lucas)", n);
        return true;
    }

    println!("{} is not prime (lucas, so it was a fermat pseudo prime)", n);
    false
}