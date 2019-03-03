pub fn mx_dif_lg(a1: Vec<&str>, a2: Vec<&str>) -> i32 {
    if a1.is_empty() || a2.is_empty() {
        return -1;
    }
    let mut size1: Vec<i32> = a1.into_iter().map(|x| (x.len() as i32)).collect();
    let mut size2: Vec<i32> = a2.into_iter().map(|x| (x.len() as i32)).collect();
    size1.sort();
    size2.sort();
    let min1 = size1[0];
    let max1 = size1[size1.len() - 1];
    let min2 = size2[0];
    let max2 = size2[size2.len() - 1];
    return std::cmp::max(max1 - min2, max2 - min1);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(a1: Vec<&str>, a2: Vec<&str>, exp: i32) -> () {
        println!("a1: {:?};", a1);
        println!("a2: {:?};", a2);
        let ans = mx_dif_lg(a1, a2);
        println!("actual:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!("{};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }

    #[test]
    fn basic_tests() {
        let mut s1 = vec![
            "hoqq",
            "bbllkw",
            "oox",
            "ejjuyyy",
            "plmiis",
            "xxxzgpsssa",
            "xxwwkktt",
            "znnnnfqknaz",
            "qqquuhii",
            "dvvvwz",
        ];
        let mut s2 = vec!["cccooommaaqqoxii", "gggqaffhhh", "tttoowwwmmww"];
        dotest(s1, s2, 13);
        s1 = vec![
            "ejjjjmmtthh",
            "zxxuueeg",
            "aanlljrrrxx",
            "dqqqaaabbb",
            "oocccffuucccjjjkkkjyyyeehh",
        ];
        s2 = vec!["bbbaaayddqbbrrrv"];
        dotest(s1, s2, 10);
        s1 = vec![
            "ccct",
            "tkkeeeyy",
            "ggiikffsszzoo",
            "nnngssddu",
            "rrllccqqqqwuuurdd",
            "kkbbddaakkk",
        ];
        s2 = vec![
            "tttxxxxxxgiiyyy",
            "ooorcvvj",
            "yzzzhhhfffaaavvvpp",
            "jjvvvqqllgaaannn",
            "tttooo",
            "qmmzzbhhbb",
        ];
        dotest(s1, s2, 14);
        s1 = vec![];
        s2 = vec!["cccooommaaqqoxii", "gggqaffhhh", "tttoowwwmmww"];
        dotest(s1, s2, -1);
        s2 = vec![];
        s1 = vec!["cccooommaaqqoxii", "gggqaffhhh", "tttoowwwmmww"];
        dotest(s1, s2, -1);
        s1 = vec![];
        s2 = vec![];
        dotest(s1, s2, -1);
    }

    extern crate rand;
    use self::rand::distributions::Alphanumeric;
    use self::rand::{thread_rng, Rng};

    fn dostr(a: i32) -> String {
        let rand_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(a as usize)
            .collect();
        return rand_string;
    }

    fn doarray(a: i32) -> Vec<String> {
        let mut rng = rand::thread_rng();
        let mut r: Vec<String> = vec![];
        for _ in 0..a {
            let s = dostr(rng.gen_range(5, 20));
            r.push(s);
        }
        return r;
    }

    fn mx_dif_lg_22(a1: Vec<&str>, a2: Vec<&str>) -> i32 {
        if a1.len() == 0 || a2.len() == 0 {
            return -1;
        }
        let mut u1: Vec<i32> = a1.into_iter().map(|x| (x.len() as i32)).collect();
        let mut u2: Vec<i32> = a2.into_iter().map(|x| (x.len() as i32)).collect();
        u1.sort();
        u2.sort();
        let max1 = u1[u1.len() - 1];
        let min1 = u1[0];
        let max2 = u2[u2.len() - 1];
        let min2 = u2[0];
        return std::cmp::max(max1 - min2, max2 - min1);
    }

    #[test]
    fn random_tests() {
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let arr1 = doarray(rng.gen_range(10, 20));
            let mut narr1: Vec<&str> = vec![];
            for i in 1..arr1.len() {
                narr1.push(&arr1[i]);
            }
            let nnarr1 = narr1.clone();
            let arr2 = doarray(rng.gen_range(5, 10));
            let mut narr2: Vec<&str> = vec![];
            for i in 1..arr2.len() {
                narr2.push(&arr2[i]);
            }
            let nnarr2 = narr2.clone();
            let sol = mx_dif_lg_22(narr1, narr2);
            dotest(nnarr1, nnarr2, sol);
        }
    }
}
