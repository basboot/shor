// greatest common divisor (recursive, Euclid)

pub fn gcd(x:u64, y:u64) -> u64 {
 
if y == 0 {

    return x ;
    
} else {
    
    return gcd(y,x % y) ;
}

}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_gcd_12_13() {
        assert_eq!(gcd(12,13),1);
    }
    
    #[test]
    fn test_gcd_12_15() {
        assert_eq!(gcd(12,15),3);
    }

    
}