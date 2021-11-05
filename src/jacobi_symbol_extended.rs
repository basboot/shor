// Helper functions for the:
//
// Baillie-PSW primality test
// The Baillie–PSW primality test is a probabilistic primality test that combines a Fermat or Miller–Rabin test 
// with a Lucas probable prime test to get a primality test that has no known counterexamples. 
// That is, there are no known composite n for which this test reports that n is probably prime.
// It has been shown that there are no counterexamples for n <2^64 (Wikipedia)
//

pub fn jacobi(a: i32, p: i32) -> i32 {
// transpilation of the Lua example from Wikipedia:
// https://en.wikipedia.org/wiki/Jacobi_symbol

    assert!(p > 0, "p must be positive");
    assert!(p % 2 == 1, "p must be odd");

    let mut n = a % p;
    let mut k = p;
    let mut t = 1;

    while n != 0 {
        while n % 2 == 0 {
            n = n / 2;
            let r = k % 8;
            if r == 3 || r == 5 { t = -t; }
            n = n % k;
        }

// switch k,n
        let temp = k;
        k = n;
        n = temp;

        if n % 4 == 3 && k % 4 == 3 { t = -t; }
        n = n % k;
    }

    if k == 1 { t } else { 0 }
}


pub fn find_first_d(n: i32) -> i32 {
// Find the first D in the sequence 5, −7, 9, −11, 13, −15, ... for which the Jacobi symbol (D/n) is −1. 

    let mut delta = 2;
    let mut d = 5;
    while jacobi(d, n) != -1 {
        d = -1 * (d + delta);
        delta = -delta;
    }
    d
}