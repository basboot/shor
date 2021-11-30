use std::fmt;
use log::debug;

type ContinuedFraction = Vec<u64>;

pub struct Fraction {
    pub numerator: u64,
    pub denominator: u64
}

// Add diplay trait to Fraction so we can print it
impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} / {}", self.numerator, self.denominator)
    }
}

pub fn cf_to_fraction(cf: ContinuedFraction, depth:usize) -> Fraction {
    assert!(depth <= cf.len(), "depth cannot be larger than the size of the continued fraction");
    assert!(depth > 0, "depth cannot be zero");

    // init at 1/0 (so we start at 0/1 after first swap)
    let mut result = Fraction {
        numerator: 1,
        denominator: 0
    };

    for i in 0..depth {
        // reverse the previous result (1/x)
        result = Fraction {
            numerator: result.denominator,
            denominator: result.numerator
        };

        debug!("Inverse fraction: {}", result);

        // add value to fraction
        result.numerator = result.numerator + cf[depth - 1 - i] * result.denominator;

        debug!("Add {} to fraction: {}", cf[depth - 1 - i], result);
    }

    return result;
}