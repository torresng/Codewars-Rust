pub fn array_diff<T: PartialEq>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    a.into_iter().filter(|x| !b.contains(x)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate rand;

    #[test]
    fn returns_expected() {
        assert_eq!(array_diff(vec![1, 2], vec![1]), vec![2]);
        assert_eq!(array_diff(vec![1, 2, 2], vec![1]), vec![2, 2]);
        assert_eq!(array_diff(vec![1, 2, 2], vec![2]), vec![1]);
        assert_eq!(array_diff(vec![1, 2, 2], vec![]), vec![1, 2, 2]);
        assert_eq!(array_diff(vec![], vec![1, 2]), vec![]);
    }

    #[test]
    fn random_testing() {
        use rand::{thread_rng, Rng};

        fn array_diff_sol<T: PartialEq>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
            a.into_iter().filter(|x| !b.contains(x)).collect()
        }

        let mut rng = thread_rng();
        for _ in 0..40 {
            let (alen, blen) = (rng.gen_range(0, 20), rng.gen_range(0, 20));
            let a: Vec<i8> = (0..alen).map(|_| rng.gen_range(-20, 20)).collect();
            let b: Vec<i8> = (0..blen).map(|_| rng.gen_range(-20, 20)).collect();
            assert_eq!(array_diff(a.clone(), b.clone()), array_diff_sol(a, b));
        }
    }
}
