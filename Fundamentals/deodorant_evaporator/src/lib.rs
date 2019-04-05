pub fn evaporator(content: f64, evap_per_day: i32, threshold: i32) -> i32 {
    let mut i = 0;
    let mut curr = content;
    while curr >= content * threshold as f64 / 100 as f64 {
        curr = curr * (1 as f64 - evap_per_day as f64 / 100.0);
        i += 1;
    }
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(_content: f64, evap_per_day: i32, threshold: i32, exp: i32) -> () {
        println!(" evap_per_day: {:?}", evap_per_day);
        println!("threshold: {:?}", threshold);
        let ans = evaporator(_content, evap_per_day, threshold);
        println!(" actual:\n{:?}", ans);
        println!("expect:\n{:?}", exp);
        println!(" {};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }

    #[test]
    fn basic_tests() {
        dotest(10.0, 10, 10, 22);
        dotest(10.0, 10, 5, 29);
        dotest(100.0, 5, 5, 59);
        dotest(50.0, 12, 1, 37);
        dotest(47.5, 8, 8, 31);
        dotest(100.0, 1, 1, 459);
        dotest(10.0, 1, 1, 459);
        dotest(100.0, 1, 5, 299);
    }

    extern crate rand;
    use self::rand::{thread_rng, Rng};

    fn evaporator_dz(_content: f64, evap_per_day: i32, threshold: i32) -> i32 {
        return (((threshold as f64) / 100.0).ln() / (1.0 - (evap_per_day as f64) / 100.0).ln())
            .ceil() as i32;
    }

    #[test]
    fn random_tests() {
        let mut rng = thread_rng();
        for _ in 0..200 {
            let content = 100.0;
            let perday = ((rng.gen_range(100, 1000) as f64) / 100.0) as i32;
            let t = ((rng.gen_range(1, 50) as f64) / 100.0) as i32;
            let threshold = t + rng.gen_range(1, 10);
            let sol = evaporator_dz(content, perday, threshold);
            dotest(content, perday, threshold, sol);
        }
    }
}
