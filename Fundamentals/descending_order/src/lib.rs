pub fn descending_order(x: u64) -> u64 {
    let mut vec: Vec<char> = x.to_string().chars().collect::<Vec<char>>();
    vec.sort_by(|a, b| b.cmp(a));
    vec.into_iter().collect::<String>().parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(descending_order(0), 0);
        assert_eq!(descending_order(1), 1);
        assert_eq!(descending_order(15), 51);
        assert_eq!(descending_order(1021), 2110);
        assert_eq!(descending_order(123456789), 987654321);
        assert_eq!(descending_order(145263), 654321);
        assert_eq!(descending_order(1254859723), 9875543221);
        assert_eq!(descending_order(102313), 332110);
        assert_eq!(descending_order(253940), 954320);
        assert_eq!(descending_order(818825), 888521);
        assert_eq!(descending_order(966972), 997662);
        assert_eq!(descending_order(152428), 854221);
        assert_eq!(descending_order(82843), 88432);
        assert_eq!(descending_order(769079), 997760);
        assert_eq!(descending_order(623092), 963220);
        assert_eq!(descending_order(578478), 887754);
        assert_eq!(descending_order(708725), 877520);
    }
}
