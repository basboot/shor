/***
Modular exponentiation, using right-to-left binary method for better performance

https://en.wikipedia.org/wiki/Modular_exponentiation

 */

pub fn modular_pow(base: u64, exponent: u64, modulus: u64) -> u64 {
    let mut exponent = exponent;
    let mut base = base;
    let mut result = 1;
    if modulus == 1 {
        return 0;
    }
    // TODO: Add assert "(modulus - 1) * (modulus - 1) cannot overflow base"

    while exponent > 0 {
        if (exponent % 2 == 1) {
            result = (result * base) % modulus;
        }
        exponent = exponent >> 1;
        base = (base * base) % modulus;
    }
    return result;
}

#[cfg(test)]
mod tests {
// Note this useful idiom: importing names from outer (for mod tests) scope.
use super::*;


//926150
#[test]
fn test_modular_pow_4294967291_2_1048573() {
assert_eq!(modular_pow(4294967291,2,1048573),926150);
}
//431550
#[test]
fn test_modular_pow_4093_5_1048573() {
assert_eq!(modular_pow(4093,5,1048573),431550);
}
//2769931358
#[test]
fn test_modular_pow_3_1125899906842624_4294967291() {
assert_eq!(modular_pow(3,1125899906842624,4294967291),2769931358);
}
//3
#[test]
fn test_modular_pow_23_1_5() {
assert_eq!(modular_pow(23,1,5),3);
}
//0
#[test]
fn test_modular_pow_7_22_7() {
assert_eq!(modular_pow(7,22,7),0);
}
//0
#[test]
fn test_modular_pow_3_40_1() {
assert_eq!(modular_pow(3,40,1),0);
}
//0
#[test]
fn test_modular_pow_0_61_62() {
assert_eq!(modular_pow(0,61,62),0);
}
}
