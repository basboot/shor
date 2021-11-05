// Helper function for the:
//
// Baillie-PSW primality test
// The Baillie–PSW primality test is a probabilistic primality test that combines a Fermat or Miller–Rabin test 
// with a Lucas probable prime test to get a primality test that has no known counterexamples. 
// That is, there are no known composite n for which this test reports that n is probably prime.
// It has been shown that there are no counterexamples for n <2^64 (Wikipedia)
//


pub fn jacobi(a:i32, p:i32) -> i32 {
// transpilation of the Lua example from Wikipedia:
// https://en.wikipedia.org/wiki/Jacobi_symbol

assert!(p > 0, "p must be positive") ;
assert!(p % 2 == 1,"p must be odd") ;

let mut n = a % p ;
let mut k = p ;
let mut t = 1 ;

while n != 0 {
   while n % 2 == 0 {
      
      n = n / 2 ;
      let r = k % 8 ;
      if r==3 || r==5 { t=-t ; }
      n = n % k ;
   }

// switch k,n
let temp = k ;
k = n ;
n = temp ;

if n % 4 == 3 && k % 4 == 3 { t = -t ; }
n = n % k ; 

}

if k==1 {t} else {0}
}


#[cfg(test)]
mod tests {
// Note this useful idiom: importing names from outer (for mod tests) scope.
use super::*;


//1
#[test]
fn test_jacobi_symbol_1783_7523() {
assert_eq!(jacobi_symbol(1783,7523),1);
}
//1
#[test]
fn test_jacobi_symbol_756479_1298351() {
assert_eq!(jacobi_symbol(756479,1298351),1);
}
//-1
#[test]
fn test_jacobi_symbol_485277_12408107() {
assert_eq!(jacobi_symbol(485277,12408107),-1);
}
//0
#[test]
fn test_jacobi_symbol_24690_12345() {
assert_eq!(jacobi_symbol(24690,12345),0);
}
//0
#[test]
fn test_jacobi_symbol_7_21() {
assert_eq!(jacobi_symbol(7,21),0);
}
//-1
#[test]
fn test_jacobi_symbol_3_41() {
assert_eq!(jacobi_symbol(3,41),-1);
}
}
