// log with base 2, float and integer versions

pub fn log2int(x: u64) -> u64 {
    let mut ans = 0;
    let mut n = 1;
    while n < x {
        ans = ans + 1;
        n = 2 * n;
    }

    ans
}

pub fn log2flt(x: f64) -> f64 {
    x.log2()
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn log2int_1024_10() {
        assert_eq!(log2int(1024), 10);
    }

    #[test]
    fn log2int_1_0() {
        assert_eq!(log2int(1), 0);
    }

    #[test]
    fn log2flt_sqrt_8() {
        assert_eq!(log2flt(8_f64.sqrt()), 1.5);
    }
}