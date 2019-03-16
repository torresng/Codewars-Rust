pub fn sum_of_n(n: i32) -> Vec<i32> {
    (0..n.abs() + 1)
        .map(|i| n.signum() * (i.pow(2) + i) / 2)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_tests() {
        assert_eq!(sum_of_n(3), vec![0, 1, 3, 6]);
        assert_eq!(sum_of_n(-4), vec![0, -1, -3, -6, -10]);
        assert_eq!(sum_of_n(1), vec![0, 1]);
        assert_eq!(sum_of_n(0), vec![0]);
        assert_eq!(sum_of_n(10), vec![0, 1, 3, 6, 10, 15, 21, 28, 36, 45, 55]);
    }

    fn re_sum_of_n(n: i32) -> Vec<i32> {
        let x = if n > 0 { 1 } else { -1 };
        (0..n.abs() + 1).fold(vec![], |mut acc, i| {
            acc.push((i + 1) * i / 2 * x);
            acc
        })
    }

    #[test]
    fn random_tests() {
        struct Rnd {
            seed: u64,
        }
        impl Rnd {
            fn new() -> Rnd {
                Rnd {
                    seed: Box::into_raw(Box::new(0)) as u64,
                }
            }
            fn gen_range(&mut self, low: u64, hi: u64) -> i32 {
                let value = low + self.seed % (hi - low);
                self.seed = (self.seed as u32) as u64 * 472_882_049_u64 % 492_876_847_u64;
                value as i32
            }
        }

        let mut n = Rnd::new();

        for _ in 0..100 {
            let i = n.gen_range(1, 100) - 30;
            assert_eq!(sum_of_n(i), re_sum_of_n(i));
        }
    }
}
