pub fn prime_factors(n: i64) -> String {
    let mut prime_numbers = Vec::new();
    let mut number = n;
    let mut prime = 2;
    let mut cnt;
    while number > 1 {
        cnt = 0;
        while number % prime == 0 {
            number /= prime;
            cnt += 1;
        }
        if cnt > 0 {
            if cnt > 1 {
                prime_numbers.push(format!("({}**{})", prime, cnt));
            } else {
                prime_numbers.push(format!("({})", prime));
            }
        }
        prime += 1;
    }
    prime_numbers.join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing(n: i64, exp: &str) -> () {
        assert_eq!(&prime_factors(n), exp)
    }

    #[test]
    fn basics_prime_factors() {
        testing(7775460, "(2**2)(3**3)(5)(7)(11**2)(17)");
        testing(17 * 17 * 93 * 677, "(3)(17**2)(31)(677)");
    }
}
