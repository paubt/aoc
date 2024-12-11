use std::fs;
use regex::Regex;


fn part1(input: String) -> String
{
    let re = Regex::new(r"mul\((?<x>\d{1,3}),(?<y>\d{1,3})\)").unwrap();
    re.captures_iter(&input)
        .fold(0, |acc, caps| acc + caps["x"].parse::<i64>().unwrap() * caps["y"].parse::<i64>().unwrap())
        .to_string() 
}


fn part2(input: String) -> String
{
    let re = Regex::new(r"(mul\((?<x>\d{1,3}),(?<y>\d{1,3})\))|(?<do>(do\(\)))|(?<dont>(don't\(\)))").unwrap();
    re.captures_iter(&input)
        .fold((0, true), |(acc_i, do_dont), caps| {
                if caps[0] == *"do()" 
                {
                    return (acc_i, true)
                }
                else if caps[0] == *"don't()"
                {
                    return (acc_i, false)
                }
                else if do_dont
                {
                    (acc_i + caps["x"].parse::<i64>().unwrap() * caps["y"].parse::<i64>().unwrap(), do_dont)
                }
                else 
                {
                    (acc_i,do_dont)
                }
            })
            .0
            .to_string()   
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

