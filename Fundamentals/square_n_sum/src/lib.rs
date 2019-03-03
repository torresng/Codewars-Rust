pub fn square_sum(vec: Vec<i32>) -> i32 {
    vec.iter().fold(0, |acc, x| acc + x * x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn returns_expected() {
        assert_eq!(square_sum(vec![1, 2]), 5);
        assert_eq!(square_sum(vec![-1, -2]), 5);
        assert_eq!(square_sum(vec![5, 3, 4]), 50);
        assert_eq!(square_sum(vec![-1, 0, 1]), 2);
        assert_eq!(square_sum(vec![-1, 1]), 2);
    }
}
