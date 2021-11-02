pub fn is_even(n: u64) -> bool {
    return (n % 2) == 0;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_even() {
        assert_eq!(is_even(2), true);
    }

    #[test]
    fn test_not_even() {
        assert_eq!(is_even(1), false);
    }
}