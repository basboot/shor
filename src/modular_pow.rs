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