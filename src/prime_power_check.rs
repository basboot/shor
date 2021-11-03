pub fn prime_power_check(n:u64) -> bool {
 //returns true if n is prime^exp with exp>1, false otherwise   

unimplemented!();
   
// true or false 
   

}






#[cfg(test)]
mod tests {
// Note this useful idiom: importing names from outer (for mod tests) scope.
use super::*;


//18446744030759878681
#[test]
fn test_prime_power_check_4294967291_2() {
assert_eq!(prime_power_check(18446744030759878681),true);
}
//1148705560180903693
#[test]
fn test_prime_power_check_4093_5() {
assert_eq!(prime_power_check(1148705560180903693),true);
}
//17001416405572203977
#[test]
fn test_prime_power_check_137_9() {
assert_eq!(prime_power_check(17001416405572203977),true);
}
//11592836324538749809
#[test]
fn test_prime_power_check_23_14() {
assert_eq!(prime_power_check(11592836324538749809),true);
}
//3909821048582988049
#[test]
fn test_prime_power_check_7_22() {
assert_eq!(prime_power_check(3909821048582988049),true);
}
//12157665459056928801
#[test]
fn test_prime_power_check_3_40() {
assert_eq!(prime_power_check(12157665459056928801),true);
}
//2305843009213693952
#[test]
fn test_prime_power_check_2_61() {
assert_eq!(prime_power_check(2305843009213693952),true);
}
}
