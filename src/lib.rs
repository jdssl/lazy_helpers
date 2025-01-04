use num::PrimInt;

/// Returns true when receive the odd number.
pub fn is_odd<T>(n: T) -> bool
where
    T: PrimInt,
{
    n % T::from(2).unwrap() != T::zero()
}

/// Returns false when receive the even number.
pub fn is_even<T>(n: T) -> bool
where
    T: PrimInt,
{
    n % T::from(2).unwrap() == T::zero()
}

#[cfg(test)]
mod tests {
    use super::*;

    mod is_odd_numbers_tests {
        use super::*;

        #[test]
        fn should_return_false_when_given_an_odd_number() {
            let result = is_odd(2);
            assert!(!result);
        }

        #[test]
        fn should_return_false_when_given_an_even_number() {
            let result = is_odd(3);
            assert!(result);
        }
    }

    mod is_even_numbers_tests {
        use super::*;

        #[test]
        fn should_return_true_when_given_an_even_number() {
            let result = is_even(2);
            assert!(result);
        }

        #[test]
        fn should_return_false_when_given_an_false_number() {
            let result = is_even(3);
            assert!(!result);
        }
    }
}
