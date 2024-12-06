use std::{fs, iter::zip};



fn part1(input: String) -> String
{
    // let (mut l, mut r): (Vec<i64>,Vec<i64>) = input
    //     .lines()
    //     .map(|l| {
    //         let sw: Vec<&str> = l.split_whitespace().collect();
    //         assert!(sw.len() == 2);
    //         (sw[0].parse::<i64>().unwrap(), sw[1].parse::<i64>().unwrap())})
    //     .unzip();
    input
}

fn part2(input: String) -> String
{
    input
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


