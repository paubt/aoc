use std::fs;

fn calc_captcha1(v: &Vec<u32>) -> u32 {
    let mut sum: u32 = 0;
    for idx in 0..v.len() - 1 {
        if v[idx] == v[idx + 1] {
            sum += v[idx];
        }
    }
    if v[0] != v[v.len() - 1] {
        sum
    } else {
        sum + v[0]
    }
}

fn calc_captcha2(v: &Vec<u32>) -> u32 {
    let mut sum: u32 = 0;
    for idx in 0..v.len() / 2 {
        if v[idx] == v[idx + v.len() / 2] {
            sum += v[idx];
        }
    }
    for idx in v.len() / 2..v.len() {
        if v[idx] == v[idx - v.len() / 2] {
            sum += v[idx];
        }
    }
    sum
}
fn main() {
    let code = fs::read_to_string("../data/day1/input.txt")
        .unwrap()
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    println!(
        "day 1 solution part 1: {} part 2: {}",
        calc_captcha1(&code),
        calc_captcha2(&code)
    );
}

#[cfg(test)]
mod tests {
    mod part1 {
        use super::super::*;
        #[test]
        fn example_1() {
            let v: Vec<u32> = vec![1, 1, 2, 2];
            assert_eq!(3, calc_captcha1(&v));
        }

        #[test]
        fn example_2() {
            let v: Vec<u32> = vec![1, 1, 1, 1];
            assert_eq!(4, calc_captcha1(&v));
        }

        #[test]
        fn example_3() {
            let v: Vec<u32> = vec![1, 2, 3, 4];
            assert_eq!(0, calc_captcha1(&v));
        }

        #[test]
        fn example_4() {
            let v: Vec<u32> = vec![9, 1, 2, 1, 2, 1, 2, 9];
            assert_eq!(9, calc_captcha1(&v));
        }
    }
    mod part2 {
        use super::super::*;
        #[test]
        fn example_21() {
            let v: Vec<u32> = vec![1, 2, 1, 2];
            assert_eq!(6, calc_captcha2(&v));
        }

        #[test]
        fn example_22() {
            let v: Vec<u32> = vec![1, 2, 2, 1];
            assert_eq!(0, calc_captcha2(&v));
        }
        #[test]
        fn example_23() {
            let v: Vec<u32> = vec![1, 2, 3, 4, 2, 5];
            assert_eq!(4, calc_captcha2(&v));
        }

        #[test]
        fn example_24() {
            let v: Vec<u32> = vec![1, 2, 3, 1, 2, 3];
            assert_eq!(12, calc_captcha2(&v));
        }

        #[test]
        fn example_25() {
            let v: Vec<u32> = vec![1, 2, 1, 3, 1, 4, 1, 5];
            assert_eq!(4, calc_captcha2(&v));
        }
    }
}
