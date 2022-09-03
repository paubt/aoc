use std::fs;

fn part1(input: &str) -> u64 {
    let t = input.lines().fold(0, |acc, line| {
        let max: u64 = line
            .split_whitespace()
            .map(|ele| ele.parse::<u64>().unwrap())
            .max()
            .unwrap();
        let min: u64 = line
            .split_whitespace()
            .map(|ele| ele.parse::<u64>().unwrap())
            .min()
            .unwrap();
        acc + (max - min)
    });
    t
}
fn part2(input: &str) -> u64 {
    let t = input.lines().fold(0, |acc, line| {
        let t: Vec<f64> = line
            .split_whitespace()
            .map(|ele| ele.parse::<f64>().unwrap())
            .collect();
        let mut sum: u64 = 0;
        'outta: for ele1 in t.iter() {
            for ele2 in t.iter().filter(|x| *x != ele1) {
                if ((ele1 / ele2) as f64 % 1.0) == 0.0 {
                    sum = (ele1 / ele2) as u64;
                    break 'outta;
                }
            }
        }
        acc + sum
    });
    t
}

fn main() {
    let input = fs::read_to_string("../data/day2/input.txt").unwrap();
    println!(
        "day 2 solution part 1: {} part 2: {}",
        part1(&input),
        part2(&input)
    );
}
