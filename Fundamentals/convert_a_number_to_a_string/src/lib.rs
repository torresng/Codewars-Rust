pub fn number_to_string(i: i32) -> String {
    i.to_string()
}

#[cfg(test)]
mod tests {
    /*
     *use number_to_string;
     */
    use super::*;

    #[test]
    fn returns_number_as_a_string() {
        assert_eq!(number_to_string(67), "67");
        assert_eq!(number_to_string(79585), "79585");
        assert_eq!(number_to_string(1 + 2), "3");
        assert_eq!(number_to_string(1 - 2), "-1");
    }
}
