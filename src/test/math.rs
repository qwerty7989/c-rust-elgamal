use crate::model::math::*;
mod malmod {
    use super::mul_mod;

    #[test]
    fn test_mul_mod_small_values() {
        assert_eq!(mul_mod(5, 3, 7), 1);
        assert_eq!(mul_mod(12, 9, 11), 6);
    }

    #[test]
    fn test_mul_mod_large_values() {
        assert_eq!(mul_mod(123456, 78910, 1000), 76);
    }

    #[test]
    fn test_mul_mod_zero_values() {
        assert_eq!(mul_mod(0, 10, 5), 0);
        assert_eq!(mul_mod(7, 0, 13), 0);
    }

    #[test]
    fn test_mul_mod_negative_values() {
        assert_eq!(mul_mod(-5, 4, 11), 2);
    }

    #[test]
    fn test_mul_mod_modulus_one() {
        assert_eq!(mul_mod(13, 45, 1), 0);
    }
}


mod fastexp {
    use super::fast_exponential;

    #[test]
    fn test_fast_exponential_small_values() {
        assert_eq!(fast_exponential(5, 3, 13), 8);
        assert_eq!(fast_exponential(2, 10, 7), 1);
    }

    #[test]
    fn test_fast_exponential_large_values() {
        assert_eq!(fast_exponential(123456, 78910, 1000), 376);
    }

    // #[test]
    // fn test_fast_exponential_zero_values() {
    //     assert_eq!(fast_exponential(0, 10, 5), 0);
    //     assert_eq!(fast_exponential(7, 0, 13), 1);
    // }

    #[test]
    fn test_fast_exponential_negative_values() {
        assert_eq!(fast_exponential(-5, 4, 11), 6);
    }

    #[test]
    fn test_fast_exponential_modulus_one() {
        assert_eq!(fast_exponential(13, 45, 1), 0);
    }
}

mod gcd {
    use super::gcd;

    #[test]
    fn test_gcd_small_values() {
        assert_eq!(gcd(5, 3), 1);
        assert_eq!(gcd(12, 9), 3);
    }

    #[test]
    fn test_gcd_large_values() {
        assert_eq!(gcd(123456, 78910), 2);
    }

    #[test]
    fn test_gcd_zero_values() {
        assert_eq!(gcd(0, 10), 10);
        assert_eq!(gcd(7, 0), 7);
    }

    #[test]
    fn test_gcd_negative_values() {
        assert_eq!(gcd(-5, 4), 1);
    }
}

mod extended_gcd {
    use super::extended_gcd;

    #[test]
    fn test_extended_gcd_small_values() {
        assert_eq!(extended_gcd(5, 3), (1, -1));
        assert_eq!(extended_gcd(12, 9), (3, 1));
    }

    #[test]
    fn test_extended_gcd_large_values() {
        assert_eq!(extended_gcd(123456, 78910), (2, -3081));
    }

    #[test]
    fn test_extended_gcd_zero_values() {
        assert_eq!(extended_gcd(0, 10), (10, 0));
        assert_eq!(extended_gcd(7, 0), (7, 0));
    }

    #[test]
    fn test_extended_gcd_negative_values() {
        assert_eq!(extended_gcd(-5, 4), (1, 1));
    }
}

mod find_mod_invert {
    use super::find_mod_invert;

    #[test]
    fn test_find_mod_invert_small_values() {
        assert_eq!(find_mod_invert(3, 11), 4);
        assert_eq!(find_mod_invert(2, 7), 4);
    }

    #[test]
    fn test_find_mod_invert_large_values() {
        assert_eq!(find_mod_invert(123456, 78910), 1);
    }

    #[test]
    fn test_find_mod_invert_zero_values() {
        assert_eq!(find_mod_invert(0, 10), 0);
        assert_eq!(find_mod_invert(7, 0), 0);
    }

    #[test]
    fn test_find_mod_invert_negative_values() {
        assert_eq!(find_mod_invert(-5, 4), 3);
    }
}

mod modular {
    use super::modular;

    #[test]
    fn test_modular_small_values() {
        assert_eq!(modular(5, 3), 2);
        assert_eq!(modular(12, 9), 3);
    }

    #[test]
    fn test_modular_large_values() {
        assert_eq!(modular(123456, 78910), 44546);
    }

    #[test]
    fn test_modular_zero_values() {
        assert_eq!(modular(0, 10), 0);
        assert_eq!(modular(7, 0), 0);
    }

    #[test]
    fn test_modular_negative_values() {
        assert_eq!(modular(-5, 4), 3);
    }
}
