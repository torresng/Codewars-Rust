pub fn opposite(number: i32) -> i32 {
    -number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(opposite(1), -1);
        assert_eq!(opposite(-1), 1);
    }

    #[test]
    fn test_positive() {
        for i in 0..100 {
            assert_eq!(opposite(i), -i);
        }
    }

    #[test]
    fn test_negative() {
        for i in -100..0 {
            assert_eq!(opposite(i), -i);
        }
    }
}
