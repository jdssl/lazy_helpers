use std::str::FromStr;

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

/// Returns data converted from &str to int
pub fn convert_str_to_int<T>(str: &str) -> T
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    str.trim()
        .parse::<T>()
        .expect("Failed to convert str to int")
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

    mod convert_str_to_int_tests {
        use std::panic;

        use super::*;

        #[test]
        fn should_return_value_converted_when_given_a_i8() {
            let result: i8 = convert_str_to_int("127");
            assert_eq!(result, 127);
        }

        #[test]
        fn should_return_value_converted_when_given_a_i16() {
            let result: i32 = convert_str_to_int("32767");
            assert_eq!(result, 32767);
        }

        #[test]
        fn should_return_value_converted_when_given_a_i32() {
            let result: i32 = convert_str_to_int("2147483647");
            assert_eq!(result, 2147483647);
        }

        #[test]
        fn should_return_value_converted_when_given_a_i64() {
            let result: i64 = convert_str_to_int("9223372036854775807");
            assert_eq!(result, 9223372036854775807);
        }

        #[test]
        fn should_return_value_converted_when_given_a_i128() {
            let result: i128 = convert_str_to_int("170141183460469231731687303715884105727");
            assert_eq!(result, 170141183460469231731687303715884105727);
        }

        #[test]
        fn should_panic_parse_int_error_when_conversion_fails() {
            let result = panic::catch_unwind(|| {
                convert_str_to_int::<i32>("three");
            });

            assert!(result.is_err(), "Expected a panic but it didn't occur.");

            if let Err(err) = result {
                let err_message = err.downcast_ref::<String>().unwrap();
                assert_eq!(
                    err_message,
                    "Failed to convert str to int: ParseIntError { kind: InvalidDigit }"
                );
            }
        }
    }
}
