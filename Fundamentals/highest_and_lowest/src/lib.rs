pub fn high_and_low(numbers: &str) -> String {
    use std::cmp;
    let f = |(max, min), x| (cmp::max(max, x), cmp::min(min, x));
    let ans = numbers
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .fold((i32::min_value(), i32::max_value()), f);
    format!("{} {}", ans.0, ans.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    // The rusts stdlib doesn't include functions for random number generation.
    // This typically provided by the rand external library, we unfortunately
    // don't have access to external libraries. We get around this by linking to
    // rand() in the C standard library.
    extern "C" {
        fn rand() -> isize;
    }

    // Safe wrapper for rand()
    fn crand() -> isize {
        unsafe { rand() as isize }
    }

    fn crand_range(range: std::ops::Range<isize>) -> isize {
        crand() % (range.end - range.start) + range.start
    }

    #[test]
    fn random_test() {
        for _ in 0..20 {
            let n = (0..crand_range(1..40))
                .map(|_| crand_range(0..1024) - 512)
                .collect::<Vec<_>>();

            let min_max = format!("{} {}", n.iter().max().unwrap(), n.iter().min().unwrap());

            let mut s = String::new();
            for i in n.iter().enumerate() {
                if i.0 != 0 {
                    s.push(' ');
                }
                s.push_str(&i.1.to_string()[..])
            }

            assert_eq!(min_max, high_and_low(&s[..]));
        }
    }

    #[test]
    fn sample_test() {
        assert_eq!("42 -9", high_and_low("8 3 -5 42 -1 0 0 -9 4 7 4 -4"));
    }

    #[test]
    fn some_test() {
        assert_eq!(
            "542 -214",
            high_and_low("4 5 29 54 4 0 -214 542 -64 1 -3 6 -6")
        );
    }

    #[test]
    fn sort_test() {
        assert_eq!("10 -10", high_and_low("10 2 -2 -10"));
    }

    #[test]
    fn plus_minus_test() {
        assert_eq!("1 -1", high_and_low("1 -1"));
    }

    #[test]
    fn plus_plus_test() {
        assert_eq!("1 1", high_and_low("1 1"));
    }

    #[test]
    fn minus_minus_test() {
        assert_eq!("-1 -1", high_and_low("-1 -1"));
    }

    #[test]
    fn plus_minus_zero_test() {
        assert_eq!("1 -1", high_and_low("1 -1 0"));
    }

    #[test]
    fn plus_plus_zero_test() {
        assert_eq!("1 0", high_and_low("1 1 0"));
    }

    #[test]
    fn minus_minus_zero_test() {
        assert_eq!("1 0", high_and_low("1 1 0"));
    }

    #[test]
    fn single_test() {
        assert_eq!("42 42", high_and_low("42"));
    }
}
