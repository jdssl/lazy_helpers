use num::PrimInt;

pub fn is_odd<T>(n: T) -> bool
where
    T: PrimInt,
{
    n % T::from(2).unwrap() != T::zero()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = is_odd(2);
        assert!(!result);
    }

    #[test]
    fn it_works_with_odd_number() {
        let result = is_odd(3);
        assert!(result);
    }
}
