pub fn count_sheep(sheep: &[bool]) -> u8 {
    sheep.iter().filter(|&&x| x).count() as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_correct_sheep_count() {
        assert_eq!(count_sheep(&[false]), 0);
        assert_eq!(count_sheep(&[true]), 1);
        assert_eq!(count_sheep(&[true, false]), 1);
    }
}
