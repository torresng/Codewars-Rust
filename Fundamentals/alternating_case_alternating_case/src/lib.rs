pub fn to_alternating_case(s: &str) -> String {
	let mut result = String::with_capacity(s.len());
	for c in s.chars() {
		if c.is_lowercase() {
			result.extend(c.to_uppercase());
		} else {
			result.extend(c.to_lowercase());
		}
	}
	result
}

#[cfg(test)]
extern crate rand;

#[cfg(test)]
mod tests {
    use super::*;
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};

    // Test solution without unicode support
    fn test_solution(s: &str) -> String {
        s.chars()
            .map(|c| {
                if c.is_lowercase() {
                    c.to_ascii_uppercase()
                } else if c.is_uppercase() {
                    c.to_ascii_lowercase()
                } else {
                    c
                }
            })
            .collect()
    }

    #[test]
    fn example_tests() {
        assert_eq!("HELLO WORLD", to_alternating_case("hello world"));
        assert_eq!("hello world", to_alternating_case("HELLO WORLD"));
        assert_eq!("HELLO world", to_alternating_case("hello WORLD"));
        assert_eq!("hEllO wOrld", to_alternating_case("HeLLo WoRLD"));
        assert_eq!(
            "Hello World",
            to_alternating_case(&to_alternating_case("Hello World")[..])
        );
        assert_eq!("12345", to_alternating_case("12345"));
        assert_eq!("1A2B3C4D5E", to_alternating_case("1a2b3c4d5e"));
        assert_eq!(
            "sTRING.tOaLTERNATINGcASE",
            to_alternating_case("String.ToAlternatingCase")
        );
    }

    #[test]
    fn random_tests() {
        for _ in 0..50 {
            let mut rng = thread_rng();
            let l = rng.gen_range(5, 50);
            let s: String = rng.sample_iter(&Alphanumeric).take(l).collect();

            assert_eq!(test_solution(&s[..]), to_alternating_case(&s[..]));
        }
    }
}
