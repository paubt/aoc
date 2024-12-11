use std::fs;
use regex::Regex;


fn part1(input: String) -> String
{
    "todo".to_string()
}


fn part2(input: String) -> String
{
    let re = Regex::new(r"mul\((?<x>\d{1,3}),(?<y>\d{1,3})\)").unwrap();
    re.captures_iter(&input).fold(0, |acc, (x,y)| )   
}

fn read_in_input(path: &str) -> String 
{
    fs::read_to_string(path).unwrap()
}

fn main() 
{
    let input = read_in_input("./data/test.txt");
    println!(
        "day 3 solution part 1: {} part 2: {}",
        part1(input.clone()),
        part2(input)
    );
}

