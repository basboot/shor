use std::collections::HashSet;

const N_SIEVE: usize = 1000; // size of the seive = max prime + 1

/***
    Find all primes < N_SIEVE using a sieve of Eratosthenes

    return: set with primes
 */
pub fn sieve_of_eratosthenes() -> HashSet<usize>{
    // Init sieve, mark every number as true (possible prime)
    let mut sieve: [bool; N_SIEVE] = [true; N_SIEVE];
    // Create empty set to return primes
    let mut primes = HashSet::new();
    // Add trivial prime to set
    primes.insert(1);

    // Use sieve to add non trivial primes to set
    for i in 2..N_SIEVE {
        // prime found
        if sieve[i] {
            // add prime to set
            primes.insert(i);
            //println!("Prime found {}", i);

            // update sieve
            let mut no_prime_number = i + i;
            while no_prime_number < N_SIEVE {
                sieve[no_prime_number] = false;
                no_prime_number += i;
            }
        }
    }

    return primes;
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;



    #[test]
    fn test_prime() {
        let h = 23_i8;
        assert_eq!(sieve_of_eratosthenes().contains(&(13_usize)), true);
    }

    #[test]
    fn test_not_prime() {
        assert_eq!(sieve_of_eratosthenes().contains(&(14_usize)), false);
    }
}