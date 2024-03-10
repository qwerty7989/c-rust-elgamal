use num_bigint::{BigInt, Sign};

/// Calculates the greatest common divisor (GCD) of two integers and the Bézout coefficients. 
///
/// The extended Euclidean algorithm is used to efficiently calculate the GCD of two integers
/// 'a' and 'b'. It also returns the Bézout coefficients 'x' and 'y', which satisfy the equation:
///
/// gcd(a, b) = a * x + b * y
///
/// # Arguments
///
/// * `n1`: The first integer.
/// * `n2`: The second integer.
///
/// # Returns
///
/// A tuple containing: 
///    * The greatest common divisor (GCD) of `n1` and `n2`.
///    * The Bézout coefficient 'x'. 
///  
/// # Examples 
///
/// ```
/// let (gcd, x) = extended_gcd(36, 60);
/// assert_eq!(gcd, 12);
/// assert_eq!(12, 36 * (-1) + 60 * 1); // Bézout's identity
/// ```
pub fn extended_gcd(n1: &BigInt, n2: &BigInt) -> (BigInt, BigInt) {
    // gcd = xa+yb
    if n2 == &BigInt::from(0) {
        (BigInt::from(1), BigInt::from(0))
    } else {
        let (x, y) = extended_gcd(n2, &(n1 % n2));
        let y_clone = y.clone();
        (y, x - n1 / n2 * y_clone)
    }
}

/// Calculates the modular exponentiation of a base 'a' raised to the power 'b' modulo 'n'.
///
/// This function implements  the fast exponentiation (also known as exponentiation by squaring) 
/// algorithm, which efficiently computes a^b mod n by repeatedly squaring the base and reducing 
/// modulo n at each step.
///
/// # Arguments
///
/// * `a`: The base integer.
/// * `b`: The exponent integer.
/// * `n`: The modulus integer.
///
/// # Returns
///
/// The result of the modular exponentiation (a ^ b) % n.
///
/// # Examples
///
/// ```
/// let result = fast_exponential(5, 3, 13);
/// assert_eq!(result, 8); // (5 ^ 3) % 13 = 125 % 13 = 8 
/// ```
pub fn fast_exponential(a: &BigInt, b: &BigInt, m: &BigInt) -> BigInt {
    if b == &BigInt::from(0) {
        return BigInt::from(1);
    }
    if b == &BigInt::from(1) {
        modular(&a, &m)
    } else if modular(b, &BigInt::from(2)) == BigInt::from(0) {
        let x = fast_exponential(&a, &(b / 2), &m);
        let x_clone = x.clone();
        modular(&(x_clone * x), &m)
    } else {
        let x = fast_exponential(&a, &(b - 1), &m);
        let a_clone = a.clone();
        let x_clone = x.clone();
        modular(&(a_clone * x_clone), &m)
    }
}
/// Calculates the modular multiplicative inverse of an integer 'a' modulo 'p'.
///
/// The modular inverse of an integer 'a' modulo 'p' is a number 'x' such that:
///   (a * x) % p = 1
///
/// The function uses the extended Euclidean algorithm to efficiently find the modular inverse. 
/// It assumes that the modular inverse exists, which is true if and only if 'a' and 'p' 
/// are relatively prime (i.e., their greatest common divisor is 1).
///
/// # Arguments
///
/// * `a`: The integer for which to find the modular inverse.
/// * `p`: The modulus.
///
/// # Returns
///
/// The modular multiplicative inverse of 'a' modulo 'p', if it exists.
///
/// # Examples
///
/// ```
/// let inverse = find_mod_invert(3, 11);
/// assert_eq!(inverse, 4); // (3 * 4) % 11 = 1 
/// ```
pub fn find_mod_invert( a: &BigInt, p: &BigInt) -> BigInt {
    let (x,_y) = extended_gcd(a,p);
    
    return modular(&x, &p);
}

/// Calculates the greatest common divisor (GCD) of two integers.
///
/// This function implements the classic Euclidean algorithm, which efficiently calculates the 
/// GCD of two integers 'a' and 'b' through repeated modulo operations.
///
/// # Arguments
///
/// * `a`: The first integer.
/// * `n`: The second integer.
///
/// # Returns
///
/// The greatest common divisor (GCD) of 'a' and 'n'.
///
/// # Examples
///
/// ```
/// let result = gcd(36, 60);
/// assert_eq!(result, 12);
/// ```
pub fn gcd(a: &BigInt, n: &BigInt) -> BigInt {
    if n == &BigInt::from(0) {
        return a.clone();
    }
    return gcd(n, &modular(&a, &n));
}

/// Calculates the modulo of an integer while ensuring the result is within the valid range.
///
/// This function offers a convenient way to compute the remainder of a division while handling
/// potentially negative results. It ensures the result is always non-negative and less than the modulus. 
///
/// # Arguments
///
/// * `number`: The integer for which to find the modulo.
/// * `modular`: The modulus.
///
/// # Returns
/// 
/// The non-negative remainder of 'number' divided by 'modular'. 
///
/// # Examples
///
/// ```
/// let result = modular(-5, 4);
/// assert_eq!(result, 3); // (-5 % 4) = 3
/// ```
pub fn modular(number: &BigInt, modular: &BigInt) -> BigInt {
    let result = number % modular;
    if result.sign() == Sign::Minus {
        return result + modular;
    }
    return result;
}
