use std::{fs, iter::zip};



fn part1(input: String) -> String
{
    let (mut l, mut r): (Vec<i64>,Vec<i64>) = input
        .lines()
        .map(|l| {
            let sw: Vec<&str> = l.split_whitespace().collect();
            assert!(sw.len() == 2);
            (sw[0].parse::<i64>().unwrap(), sw[1].parse::<i64>().unwrap())})
        .unzip();
    l.sort();
    r.sort();
    zip(l.iter(), r.iter())
        .map(|(a,b)| (a-b).abs())
        .sum::<i64>()
        .to_string()
}

fn part2(input: String) -> String
{
    let (l, r): (Vec<i64>,Vec<i64>) = input
        .lines()
        .map(|l| {
            let sw: Vec<&str> = l.split_whitespace().collect();
            assert!(sw.len() == 2);
            (sw[0].parse::<i64>().unwrap(), sw[1].parse::<i64>().unwrap())})
        .unzip();
    let k: String= l
        .iter()
        .map(|n| n * i64::try_from(r.iter()
                                          .filter(|x|  *x == n)
                                          .count())
                                        .unwrap())
        .sum::<i64>()
        .to_string();
    //.sum::<usize>().to_string();
    k
}

fn read_in_input(path: &str) -> String 
{
    fs::read_to_string(path).unwrap()
}

fn main() 
{
    let input = read_in_input("./data/input.txt");
    println!(
        "day 2 solution part 1: {} part 2: {}",
        part1(input.clone()),
        part2(input)
    );
}


