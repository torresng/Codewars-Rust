pub fn solution(word: &str, ending: &str) -> bool {
    word.ends_with(ending)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(true, solution("abc", "c"));
        assert_eq!(false, solution("strawberry", "banana"));
        assert_eq!(true, solution("samurai", "ai"));
        assert_eq!(false, solution("sumo", "omo"));
        assert_eq!(true, solution("ninja", "ja"));
        assert_eq!(true, solution("sensei", "i"));
        assert_eq!(false, solution("samurai", "ra"));
        assert_eq!(false, solution("abc", "abcd"));
        assert_eq!(true, solution("abc", "abc"));
        assert_eq!(false, solution("ails", "fails"));
        assert_eq!(true, solution("fails", "ails"));
        assert_eq!(false, solution("this", "fails"));
    }
}
