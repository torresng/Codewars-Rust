pub fn divisors(integer: u32) -> Result<Vec<u32>, String> {
    let ret = (2..integer / 2 + 1)
        .filter(|x| integer % x == 0)
        .collect::<Vec<u32>>();
    if ret.len() != 0 {
        return Ok(ret);
    }
    Err(format!("{} is prime", integer))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        assert_eq!(divisors(15), Ok(vec![3, 5]));
        assert_eq!(divisors(12), Ok(vec![2, 3, 4, 6]));
        assert_eq!(divisors(253), Ok(vec![11, 23]));
        assert_eq!(divisors(24), Ok(vec![2, 3, 4, 6, 8, 12]));
        assert_eq!(divisors(13), Err("13 is prime".to_string()));
        assert_eq!(divisors(3), Err("3 is prime".to_string()));
        assert_eq!(divisors(29), Err("29 is prime".to_string()));
    }
}
