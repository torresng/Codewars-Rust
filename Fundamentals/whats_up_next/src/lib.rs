pub fn next_item<T: PartialEq<T> + Clone>(slice: &[T], find: T) -> Option<T> {
    slice.iter().skip_while(|&x| *x != find).nth(1).cloned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(next_item(&[0, 1], 0), Some(1));
        assert_eq!(next_item(&[0, 1], 1), None);
        assert_eq!(
            next_item((1..10).collect::<Vec<_>>().as_slice(), 7),
            Some(8)
        );
        assert_eq!(
            next_item("Hello, world!".chars().collect::<Vec<_>>().as_slice(), 'w'),
            Some('o')
        );
        assert_eq!(next_item(&["Joe", "Bob", "Sally"], "Bob"), Some("Sally"))
    }
}
