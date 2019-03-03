extern crate rand;

pub fn solution(phrase: &str) -> String {
    phrase.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use self::rand::Rng;
    use super::*;

    fn sol(phrase: &str) -> String {
        let mut vec: Vec<char> = phrase.chars().collect();
        vec.reverse();

        return vec.into_iter().collect();
    }

    #[test]
    fn sample_test() {
        assert_eq!(solution("world"), "dlrow");
    }

    #[test]
    fn basic_tests() {
        assert_eq!(solution("world"), "dlrow");
        assert_eq!(solution("hello"), "olleh");
        assert_eq!(solution(""), "");
        assert_eq!(solution("h"), "h");
    }

    #[test]
    fn random_tests() {
        let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();

        for _i in 0..100 {
            let mut phrase = String::new();
            let length = rand::thread_rng().gen_range(1, 1001);

            for _j in 0..length {
                phrase.push(chars[rand::thread_rng().gen_range(0, chars.len())]);
            }

            let phrase: &str = &phrase;

            assert_eq!(solution(phrase), sol(phrase));
        }
    }
}
