use crate::baillie_psw_prime;

const MAX_ROOT: u32 = 100; // maximum root to try
const EPSILON:f64 = 0.01_f64;  // maximum rounding error

// returns true if n is prime^exp with exp>1, false otherwise
pub fn prime_power_check(n: u64) -> bool {
    let mut current_root = 2;
    loop {
        // take root
        let result = (n as f64).powf(1_f64/(current_root as f64));

        println!("Prime power: ---------------------------------------");
        println!("{} => {}", current_root, result);

        // stop if it is not possible to find a valid integer result anymore
        // TODO: because we have already excluded even numbers, this can be raised to 3
        if result < (2_f64 - EPSILON) {
            println!("Result {} is too small, abort", result);
            break;
        }

        // is result an integer?
        if (result - result.round()).abs() <= EPSILON {
            println!("Result {} is an integer", result);
            let result = result.round() as u64;
            println!("Result {} as integer", result);

            // check if rounded result is valid
            if result.pow(current_root) == n {
                println!("Rounded result is valid");
                // check if result is prime
                if baillie_psw_prime(result) {
                    println!("{} is a prime power, because {}^{}={}", n, result, current_root, n);
                    return true;
                } else {
                    println!("{} is not prime, so continue", result);
                }
            } else {
                println!("Rounded result is invalid, because {}^{}={}, not {}", result, current_root, result.pow(current_root), n);
            }
        }

        // next root
        current_root += 1;
        // possibly abort
        if current_root > MAX_ROOT {
            println!("Prime power check aborted (MAX_ROOT reached)");
            break;
        }
    }

    println!("{} is not a prime power", n);
    false
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;


    //18446744030759878681
    // Disabled test, because it overflows Lucas
    // #[test]
    // fn test_prime_power_check_4294967291_2() {
    //     assert_eq!(prime_power_check(18446744030759878681), true);
    // }

    //1148705560180903693
    #[test]
    fn test_prime_power_check_4093_5() {
        assert_eq!(prime_power_check(1148705560180903693), true);
    }

    //17001416405572203977
    #[test]
    fn test_prime_power_check_137_9() {
        assert_eq!(prime_power_check(17001416405572203977), true);
    }

    //11592836324538749809
    #[test]
    fn test_prime_power_check_23_14() {
        assert_eq!(prime_power_check(11592836324538749809), true);
    }

    //3909821048582988049
    #[test]
    fn test_prime_power_check_7_22() {
        assert_eq!(prime_power_check(3909821048582988049), true);
    }

    //12157665459056928801
    #[test]
    fn test_prime_power_check_3_40() {
        assert_eq!(prime_power_check(12157665459056928801), true);
    }

    //2305843009213693952
    #[test]
    fn test_prime_power_check_2_61() {
        assert_eq!(prime_power_check(2305843009213693952), true);
    }

    //4294967291
    #[test]
    fn test_prime_power_check_4294967291_1() {
        assert_eq!(prime_power_check(4294967291), false);
    }

    //1048573
    #[test]
    fn test_prime_power_check_1048573_1() {
        assert_eq!(prime_power_check(1048573), false);
    }

    //131621703842267136
    #[test]
    fn test_prime_power_check_6_22() {
        assert_eq!(prime_power_check(131621703842267136), false);
    }

    //6221821273427820544
    #[test]
    fn test_prime_power_check_22_14() {
        assert_eq!(prime_power_check(6221821273427820544), false);
    }

    //9042293113509953753
    #[test]
    fn test_prime_power_check_2083337_3() {
        assert_eq!(prime_power_check(9042293113509953753), false);
    }
}
