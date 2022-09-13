use std::collections::HashSet;
use std::fs;

fn part1(input: &String) -> u64 {
    input.lines().fold(0, |acc, l| {
        let mut hshst = HashSet::new();
        for ele in l.split_whitespace() {
            match hshst.insert(ele) {
                true => continue,
                false => return acc,
            }
        }
        acc + 1
    })
}

fn part2(input: &String) -> u64 {
    input.lines().fold(0, |acc, l| {
        let mut hshst: HashSet<[u32; 26]> = HashSet::new();
        for ele in l.split_whitespace() {
            let mut word_decomposed = [0; 26];
            for c in ele.chars() {
                word_decomposed[c as usize - 97] += 1;
            }
            match hshst.insert(word_decomposed) {
                true => continue,
                false => return acc,
            }
        }
        acc + 1
    })
}

fn main() {
    let input = fs::read_to_string("../data/day4/input.txt").unwrap();

    println!(
        "day 2 solution part 1: {} part 2: {}",
        part1(&input),
        part2(&input)
    );
}
