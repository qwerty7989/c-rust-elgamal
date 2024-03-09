use crate::model::prime::*;

mod is_prime {
    use super::is_prime;

    #[test]
    fn test_is_prime_small_values() {
        assert_eq!(is_prime(2), true);
        assert_eq!(is_prime(3), true);
        assert_eq!(is_prime(5), true);
        assert_eq!(is_prime(7), true);
        assert_eq!(is_prime(11), true);
        assert_eq!(is_prime(13), true);
        assert_eq!(is_prime(17), true);
        assert_eq!(is_prime(19), true);
        assert_eq!(is_prime(23), true);
        assert_eq!(is_prime(29), true);
        assert_eq!(is_prime(31), true);
        assert_eq!(is_prime(37), true);
        assert_eq!(is_prime(41), true);
        assert_eq!(is_prime(43), true);
        assert_eq!(is_prime(47), true);
        assert_eq!(is_prime(53), true);
        assert_eq!(is_prime(59), true);
        assert_eq!(is_prime(61), true);
        assert_eq!(is_prime(67), true);
        assert_eq!(is_prime(71), true);
        assert_eq!(is_prime(73), true);
        assert_eq!(is_prime(79), true);
        assert_eq!(is_prime(83), true);
        assert_eq!(is_prime(89), true);
        assert_eq!(is_prime(97), true);
    }

    #[test]
    fn test_is_prime_large_values() {
        assert_eq!(is_prime(1009), true);
        assert_eq!(is_prime(1013), true);
        assert_eq!(is_prime(1019), true);
        assert_eq!(is_prime(1021), true);
        assert_eq!(is_prime(1031), true);
        assert_eq!(is_prime(1033), true);
        assert_eq!(is_prime(1039), true);
        assert_eq!(is_prime(1049), true);
        assert_eq!(is_prime(1051), true);
        assert_eq!(is_prime(1061), true);
        assert_eq!(is_prime(1063), true);
        assert_eq!(is_prime(1069), true);
        assert_eq!(is_prime(1087), true);
        assert_eq!(is_prime(1091), true);
        assert_eq!(is_prime(1093), true);
        assert_eq!(is_prime(1097), true);
    }
}
