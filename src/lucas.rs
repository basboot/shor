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


pub fn u_k(k: u64, n: i64, p: i64, q: i64) -> i64 {

    let d = p * p -4* q; // TODO: why is d unused?
    if k == 1 {
        return 1;
    } else if k % 2 == 0 {
        let k2 = k / 2;
        let mut val = u_k(k2, n, p, q) * v_k(k2, n, p, q);
        val = my_mod(val, n);
        return val;
    } else {
        let k2 = k - 1;
        let mut val = p * u_k(k2, n, p, q) + v_k(k2, n, p, q);
        if val % 2 == 1 {
            val += n ;
        }
        val = val / 2;
        val = my_mod(val, n);
        return val;
    }
}

pub fn v_k(k: u64, n: i64, p: i64, q: i64) -> i64 {

    let d = p * p -4* q;
    if k == 1 {
        return p;
    } else if k % 2 == 0 {
        let k2 = k / 2;
        let tmpvk = v_k(k2, n, p, q);
        let tmpuk = u_k(k2, n, p, q);
        let mut val = tmpvk * tmpvk + d * tmpuk * tmpuk;
         if val % 2 == 1 {
            val += n ;
        }
        val = val / 2 ;
        return my_mod(val,n);
    } else {
        let k2 = k - 1;
        let mut val = d * u_k(k2, n, p, q) + p * v_k(k2, n, p, q);
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
        assert_eq!(u_k(20, 19, 3, -1), 0);
    }

    #[test]
    fn lucastest_pseudo_prime() {
        assert_eq!(u_k(120, 119, 3, -1), 0);
    }

    #[test]
    fn lucastest_composite() {
        assert!(u_k(222, 221, 3, -1) != 0);
    }
    
}


