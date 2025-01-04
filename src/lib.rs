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

/// Returns data converted from &str to i32
pub fn convert_str_to_i32(str: &str) -> i32 {
    str.trim()
        .parse::<i32>()
        .expect("Failed to convert str to i32")
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

    mod convert_str_to_i32_tests {
        use std::panic;

        use super::*;

        #[test]
        fn should_return_value_converted_when_given_a_number() {
            let result = convert_str_to_i32("2");
            assert_eq!(result, 2);
        }

        #[test]
        fn should_panic_parse_int_error_when_conversion_fails() {
            let result = panic::catch_unwind(|| {
                convert_str_to_i32("three");
            });

            assert!(result.is_err(), "Expected a panic but it didn't occur.");

            if let Err(err) = result {
                let err_message = err.downcast_ref::<String>().unwrap();
                assert_eq!(
                    err_message,
                    "Failed to convert str to i32: ParseIntError { kind: InvalidDigit }"
                );
            }
        }
    }
}
