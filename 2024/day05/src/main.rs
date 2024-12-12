use std::fs;

fn part1(input: String) -> String
{
    "TODO".to_string()
}

fn part2(input: String) -> String
{
    "TODO".to_string()
}

fn read_in_input(path: &str) -> String 
{
    fs::read_to_string(path).unwrap()
}

fn main() 
{
    let input = read_in_input("./data/input.txt");
    println!(
        "day 3 solution part 1: {} part 2: {}",
        part1(input.clone()),
        part2(input)
    );
}

