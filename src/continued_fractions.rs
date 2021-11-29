type ContinuedFraction = Vec<u64>;


pub fn calculate_cf(x: f64, depth: u8) -> ContinuedFraction {
    let mut vec = Vec::new();
    let mut d = 0;
    let mut a = x as u64;
    let mut rest = x - a as f64;
    vec.push(a);

    while a != 0 && d < depth {
        a = (1.0 / rest) as u64;
        if a > 1000000 {
            break;
        }
        vec.push(a);
        rest = 1.0 / rest - a as f64;
        d += 1;
    }

    return vec;
}

pub fn print_cf(cf: &ContinuedFraction) {
    print!("[{};", cf[0]);
    for i in 1..cf.len() - 1 {
        print!("{},", cf[i]);
    }

    println!("{}]", cf[cf.len() - 1])
}

pub fn a(k:i64,cf:&ContinuedFraction) -> u64{
    if k==-1 { return 1;}
    if k==0 {return cf[0];}
    return cf[k as usize]*a(k-1,cf)+a(k-2,cf);
    
}

pub fn b(k:i64,cf:&ContinuedFraction) -> u64{
    if k==-1 { return 0;}
    if k==0 {return 1;}
    return cf[k as usize]*b(k-1,cf)+b(k-2,cf);
    
}


pub fn evaluate_cf(k:i64,cf: &ContinuedFraction) -> (u64, u64) {

    return (a(k,cf),b(k,cf));
    
}


#[cfg(test)]

mod tests {

// Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    
    
    #[test]
    fn calculate_1() {
        assert_eq!(calculate_cf(1.23456789, 8),vec![1u64,4,3,1,3,1,13565,1,8]);
    }
    
    #[test]
    fn calculate_2() {
        assert_eq!(calculate_cf(1.001953125,2),vec![1u64,512]);
    }
    
    #[test]
    fn evaluate_1() {
        assert_eq!(evaluate_cf(5,&vec![1u64,4,3,1,3,1,13565,1,8]),(100,81));
    }
    
    #[test]
    fn evaluate_2() {
        assert_eq!(evaluate_cf(5,&vec![1u64,2,3,4,5,6,7,8]),(1393,972));
    }
}


