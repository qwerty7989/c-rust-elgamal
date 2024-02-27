pub fn mul_mod( a: i128, b: i128, m: i128) -> i128{
    let mut res: i128 = 0;
    let mut x = a % m;
    let mut y = b;
    while y > 0 {
        if y&1 == 1 {
            res = modular(res + x, m)
        }

        x = modular(x * 2, m);
        y /= 2;
    }

    return modular(res, m)
}

pub fn extended_gcd( n1: i128, n2: i128) -> (i128, i128) {
    // gcd = xa+yb
    if n2 == 0 {
        return (1, 0);
    }

    let (mut  x, mut y) = extended_gcd(n2, n1%n2);
    (x, y) = (y ,x -  n1/n2 * y );
    return (x, y)
}

pub fn fast_exponential( a: i128, b: i128, n: i128 ) -> i128 {
    if b == 1 { return modular(a, n) };
    
    if b%2 == 0 {
        let x = fast_exponential(a, b/2, n);
        return mul_mod(x,x,n)
    }
    else{
        let x = fast_exponential(a, b-1, n);
        return mul_mod(x,a,n)
    }
}

pub fn find_mod_invert( a: i128, p: i128) -> i128 {
    let (x,_y) = extended_gcd(a,p);
    
    return modular(x, p);
}

/// return GCD between two number (a,n)
pub fn gcd(a: i128, n: i128) -> i128 {
    if n == 0 {
        return a;
    }
    return gcd( n, a%n);
}

/// return 
pub fn modular(number: i128, modular: i128) -> i128{
    let result = number % modular;
    if result < 0 {
        result + modular
    } else {
        result
    }
}