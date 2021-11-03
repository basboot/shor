/***
    Calculate if a number is a strong (Fermat) pseudoprime to base BASE
    https://en.wikipedia.org/wiki/Strong_pseudoprime
 */
use crate::is_even::is_even;

const BASE: u64 = 2; // base to calculate the pseudoprime

pub fn pseudo_prime(n: u64) -> bool{
    assert!(n > 0, "n must be positive") ;

    // the algorithm only works if n > 2
    if n < 3 {
        return true;
    }

    let modulus = n;
    let mut power = n - 1;

    loop {
        let mut result = BASE;
        for _i in 2..power + 1 {
            result = (result * BASE) % modulus;
        }
        println!("{} ^ {} mod {} = {}", BASE, power, modulus, result);

        // if the result is -1, this is a strong pseudoprime
        if result == modulus - 1 {
            return true;
        }
        // if the result is not 1 (or -1), this is not a pseudoprime
        if result != 1 {
            return false;
        }
        // continue if the exponent is still even
        if !is_even(power) {
            break
        }

        // halve the exponent
        power /= 2;
    }

    // we did not find a counterexample, so this is a pseudoprime
    return true
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_pseudo_prime() {
        let base_two_pseudo_primes = [2047, 3277, 4033, 4681, 8321, 15841, 29341, 42799, 49141, 52633, 65281, 74665, 80581, 85489, 88357, 90751];

        for p in base_two_pseudo_primes {
            assert_eq!(pseudo_prime(p), true);
        }
    }

    #[test]
    fn test_real_prime() {
        let primes = [1, 2, 3, 1733, 1741, 4001, 4003, 5779, 7919, 193939, 27644437];

        for p in primes {
            assert_eq!(pseudo_prime(p), true);
        }
    }
    #[test]
    fn test_not_pseudo_prime() {
        let no_primes = [4, 15, 1701, 3473, 6787, 7821];

        for p in no_primes {
            assert_eq!(pseudo_prime(p), false);
        }
    }
}