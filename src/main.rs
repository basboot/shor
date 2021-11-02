mod sieve_of_eratosthenes;
use sieve_of_eratosthenes::sieve_of_eratosthenes;

fn main() {
    println!("Hello, Shor!");

    let candidate: usize = 18;

    let primes = sieve_of_eratosthenes();

    if !primes.contains(&candidate) {
        println!("{} is not prime (possible candidate)", candidate);
    } else {
        println!("{} is prime (so not a possible candidate)", candidate);
    }
}
