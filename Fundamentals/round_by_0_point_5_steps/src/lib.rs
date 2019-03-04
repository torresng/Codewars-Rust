extern crate rand;

pub fn solution(n: f64) -> f64 {
    (n * 2f64).round() / 2f64
}

#[cfg(test)]
mod tests {
    /*
     *use solution;
     */
    use super::*;
    use rand;

    #[test]
    fn sample_tests() {
        assert_eq!(solution(4.2), 4.0);
        assert_eq!(solution(4.4), 4.5);
        assert_eq!(solution(4.6), 4.5);
        assert_eq!(solution(4.8), 5.0);
        assert_eq!(solution(4.75), 5.0);
    }

    #[test]
    fn random_tests() {
        for _ in 0..100 {
            let n = rand::random::<f64>() * 100.0;

            assert_eq!(solution(n), (n * 2.0).round() / 2.0);
        }
    }
}
