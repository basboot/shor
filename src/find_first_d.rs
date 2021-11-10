// Helper function for the Baillie-PSW primality test
// TODO: unit tests

//use jacobi;

pub fn find_first_d(n:i64) -> i32 {
// Find the first D in the sequence 5, −7, 9, −11, 13, −15, ... for which the Jacobi symbol (D/n) is −1. 

    let mut delta = 2 ;
    let mut d = 5 ;
    while jacobi(d,n) != -1 {
        d = -1 * (d+delta) ;
        delta = -delta ;
    }
d
}

