/*
Working Python.....
D can be calculated from P and Q
% behaves as modulo

def U(k,n,P,Q,D):
    if k == 1:
        return 1
    elif k%2 == 0:
        k2 = k//2
        val = (U(k2,n,P,Q,D)*V(k2,n,P,Q,D))%n
        return val
    else:
        k2 = k-1
        val = P*U(k2,n,P,Q,D)+V(k2,n,P,Q,D)
        if val%2 ==1:
            val+=n
        val = (val//2)%n
        return val

def V(k,n,P,Q,D):
    if k == 1:
        return P
    elif k%2 == 0:
        k2 = k//2
        val = (V(k2,n,P,Q,D)**2-2*pow(Q,k2,n))%n
        return val
    else:
        k2 = k-1
        val = D*U(k2,n,P,Q,D)+P*V(k2,n,P,Q,D)
        if val%2 ==1:
            val+=n
        val = (val//2)%n
        return val
*/

//transpiled
//careful % python != % rust !!
//lots of snake case warnings .....


fn my_mod(x: i64, n: i64) -> i64 {
    if x >= 0 {
        return x % n;
    } else {
        return n - (-x % n);
    }
}


pub fn U(k: u64, n: i64, P: i64, Q: i64) -> i64 {

    let D = P*P-4*Q;
    if k == 1 {
        return 1;
    } else if k % 2 == 0 {
        let k2 = k / 2;
        let mut val = U(k2, n, P, Q) * V(k2, n, P, Q);
        val = my_mod(val, n);
        return val;
    } else {
        let k2 = k - 1;
        let mut val = P * U(k2, n, P, Q) + V(k2, n, P, Q);
        if val % 2 == 1 {
            val += n ;
        }
        val = val / 2;
        val = my_mod(val, n);
        return val;
    }
}

pub fn V(k: u64, n: i64, P: i64, Q: i64) -> i64 {

    let D = P*P-4*Q;
    if k == 1 {
        return P;
    } else if k % 2 == 0 {
        let k2 = k / 2;
        let tmpvk = V(k2, n, P, Q);
        let tmpuk = U(k2, n, P, Q);
        let mut val = tmpvk * tmpvk + D * tmpuk * tmpuk;
         if val % 2 == 1 {
            val += n ;
        }
        val = val / 2 ;
        return my_mod(val,n);
    } else {
        let k2 = k - 1;
        let mut val = D * U(k2, n, P, Q) + P * V(k2, n, P, Q);
        if val % 2 == 1 {
            val += n ;
        }
        val = val / 2;
        val = my_mod(val, n);
        return val;
    }
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;


    #[test]
    fn my_mod_positive() {
        assert_eq!(my_mod(33, 10), 3);
    }

    #[test]
    fn my_mod_negative() {
        assert_eq!(my_mod(-33, 10), 7);
    }

    
    #[test]
    fn lucastest_prime() {
        assert_eq!(U(20, 19, 3, -1), 0);
    }

    #[test]
    fn lucastest_pseudo_prime() {
        assert_eq!(U(120, 119, 3, -1), 0);
    }

    #[test]
    fn lucastest_composite() {
        assert!(U(222, 221, 3, -1) != 0);
    }
    
}


