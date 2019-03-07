pub fn reverse_words(str: &str) -> String {
    let words = str.split(" ").collect::<Vec<&str>>();
    let mut vec = Vec::new();
    for w in &words {
        let wrev = w.chars().rev().collect::<String>();
        vec.push(wrev);
    }
    vec.join(" ")
}

#[cfg(test)]
mod tests {
    extern crate rand;
    use self::rand::Rng;
    use super::*;

    #[test]
    fn sample_test() {
        assert_eq!(
            reverse_words("The quick brown fox jumps over the lazy dog."),
            "ehT kciuq nworb xof spmuj revo eht yzal .god"
        );
        assert_eq!(reverse_words("apple"), "elppa");
        assert_eq!(reverse_words("a b c d"), "a b c d");
        assert_eq!(
            reverse_words("double  spaced  words"),
            "elbuod  decaps  sdrow"
        );
        assert_eq!(reverse_words(""), "");
        assert_eq!(reverse_words("ab   ba   cd"), "ba   ab   dc");
    }

    fn solution(str: &str) -> String {
        let words = str.split(" ").collect::<Vec<_>>();
        let mut res = "".to_string();
        for w in words {
            let mut x = w.to_string();
            x += " ";
            x = x.chars().rev().collect();
            res.push_str(&x);
        }
        res[1..].to_string()
    }

    #[test]
    fn random_tests() {
        let words = [
            "Sam  x  abc",
            "harris   yellow   black",
            "patrick z uip",
            "Feenan evan mac",
            "Cole P hop",
            "Favuzzi greg",
            "david Lendieta cucker",
            "a b c d e f",
            "Kile clooney make me",
            "marky bark dark glock",
            " ",
        ];

        for _ in 0..10 {
            let test_string = words[rand::thread_rng().gen_range(0, words.len() - 1)];
            println!("{}", test_string);
            assert_eq!(reverse_words(&test_string), solution(&test_string));
        }
    }
}
