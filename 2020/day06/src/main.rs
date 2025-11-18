use std::collections::HashSet;
use std::fs;

fn part1(input: &str) {
    let c: usize = input
        .split("\n\n")
        .map(|group| {
            group.lines().fold(HashSet::new(), |mut acc, l| {
                for c in l.chars() {
                    acc.insert(c);
                }
                acc
            })
        })
        .map(|h| h.len())
        .sum();
    println!("Part 1: {}", c);
}

fn part2(input: &str) {
    let c: usize = input
        .split("\n\n")
        .map(|group| {
            let mut lines = group.lines();
            let mut first_h = HashSet::new();
            for c in lines.next().unwrap().chars() {
                first_h.insert(c);
            }
            for l in lines {
                let h = l.chars().fold(HashSet::new(), |mut acc, c| {
                    acc.insert(c);
                    acc
                });
                first_h = first_h.intersection(&h).fold(HashSet::new(), |mut acc, c| {
                    acc.insert(*c);
                    acc
                });
            }
            first_h
        })
        .map(|h| h.len())
        .sum();
    println!("Part 1: {}", c);
}

fn main() {
    let code = fs::read_to_string("../data/day06/input.txt").unwrap();
    part1(&code);
    part2(&code);
}
