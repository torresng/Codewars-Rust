pub fn duplicate_encode(word: &str) -> String {
    let word = word.to_lowercase();
    let mut counts = std::collections::HashMap::new();
    for w in word.chars() {
        *counts.entry(w).or_insert(0) += 1;
    }
    word.chars()
        .map(|c| match *counts.get(&c).unwrap() {
            1 => '(',
            _ => ')',
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_tests() {
        assert_eq!(duplicate_encode("din"), "(((");
        assert_eq!(duplicate_encode("recede"), "()()()");
        assert_eq!(duplicate_encode("Success"), ")())())", "should ignore case");
        assert_eq!(duplicate_encode("(( @"), "))((");
    }

    #[test]
    fn special_tests() {
        assert_eq!(duplicate_encode("CodeWarrior"), "()(((())())");
        assert_eq!(duplicate_encode("Supralapsarian"), ")()))()))))()(");
        assert_eq!(
            duplicate_encode("iiiiii"),
            "))))))",
            "duplicate-only-string"
        );
        assert_eq!(duplicate_encode(" ( ( )"), ")))))(");
    }
}
