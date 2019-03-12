pub struct Cipher {
    map: Vec<(char, char)>,
}

impl Cipher {
    pub fn new(map1: &str, map2: &str) -> Cipher {
        Cipher {
            map: map1.chars().zip(map2.chars()).collect(),
        }
    }

    pub fn encode(&self, string: &str) -> String {
        string
            .chars()
            .map(|c| self.map.iter().find(|x| x.0 == c).map_or(c, |y| y.1))
            .collect()
    }

    pub fn decode(&self, string: &str) -> String {
        string
            .chars()
            .map(|c| self.map.iter().find(|x| x.1 == c).map_or(c, |y| y.0))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let map1 = "abcdefghijklmnopqrstuvwxyz";
        let map2 = "etaoinshrdlucmfwypvbgkjqxz";

        let cipher = Cipher::new(map1, map2);

        assert_eq!(cipher.encode("abc"), "eta");
        assert_eq!(cipher.encode("xyz"), "qxz");
        assert_eq!(cipher.decode("eirfg"), "aeiou");
        assert_eq!(cipher.decode("erlang"), "aikcfu");
    }

    #[test]
    fn basic() {
        let map1 = "abcdefghijklmnopqrstuvwxyz";
        let map2 = "etaoinshrdlucmfwypvbgkjqxz";

        let cipher = Cipher::new(map1, map2);

        assert_eq!(cipher.encode("abc"), "eta");
        assert_eq!(cipher.encode("xyz"), "qxz");
        assert_eq!(cipher.encode("aeiou"), "eirfg");
        assert_eq!(cipher.decode("eta"), "abc");
        assert_eq!(cipher.decode("qxz"), "xyz");
        assert_eq!(cipher.decode("eirfg"), "aeiou");
    }

    #[test]
    fn other() {
        let map1 = "abcdefghijklmnopqrstuvwxyz";
        let map2 = "tfozcivbsjhengarudlkpwqxmy";

        let cipher = Cipher::new(map1, map2);

        assert_eq!(cipher.encode("abc"), "tfo");
        assert_eq!(cipher.decode("tfo"), "abc");
        assert_eq!(cipher.encode("tjuukf"), "kjpphi");
        assert_eq!(cipher.decode("kjpphi"), "tjuukf");
        assert_eq!(cipher.decode("tjuukf"), "ajqqtb");
        assert_eq!(cipher.encode("ajqqtb"), "tjuukf");
    }

    #[test]
    fn special_characters() {
        let map1 = "abcdefghijklmnopqrstuvwxyz";
        let map2 = "tfozcivbsjhengarudlkpwqxmy";

        let cipher = Cipher::new(map1, map2);

        assert_eq!(cipher.encode("a_bc&*83"), "t_fo&*83");
        assert_eq!(cipher.decode("t_fo&*83"), "a_bc&*83");
    }
}
