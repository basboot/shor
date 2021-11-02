mod sieve_of_eratosthenes;
mod is_even;

use sieve_of_eratosthenes::sieve_of_eratosthenes;
use is_even::is_even;

fn main() {
    println!("Hello, Shor!");

    let candidate: u64 = 21;

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
}
