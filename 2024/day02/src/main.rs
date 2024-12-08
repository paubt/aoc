use std::fs;

fn check_if_is_safe(v: &Vec<i64>) -> bool
{
    // check if acending only?
    if v.is_sorted() || v.iter().rev().is_sorted(){
        v.windows(2)
            .map(|w| (w[1]-w[0]).abs())
            .fold(true,|acc, x| if x > 0 && x <= 3 {acc} else {false})
    }
    else {
        false
    }
}

fn part1(input: String) -> String
{
    let l: Vec<Vec<i64>> = input
        .lines()
        .map(|l| l.split_whitespace()
                        .map(|x| x.parse::<i64>().unwrap()).collect()).collect();
    
    l.iter().fold(0, |acc, v | if check_if_is_safe(v) {acc+1} else {acc} ).to_string()
}

fn check_if_is_safe_with_tolerance(v: &Vec<i64>) -> bool
{
    // // check if sorted
    // let k = v
    //     .windows(2)
    //     .map(|w| (w[1]-w[0]).abs())
    //     .collect::<Vec<i64>>()
    //     .windows(2)
    //     .fold((0,true), |(acc_i,acc_b),w| if acc_b && (w[0] * w[1]) > 0 {(acc_i+1, true)} else {(acc_i, false)});
    // false
    if check_if_is_safe(v) 
    {
        true
    }
    else {
        for i in 0..(v.len())
        {
            let mut temp = v.clone();
            temp.remove(i);
            if check_if_is_safe(&temp)
            {
                return true;
            }
        }
        false
    }
}

fn part2(input: String) -> String
{
    let l: Vec<Vec<i64>> = input
        .lines()
        .map(|l| l.split_whitespace()
                        .map(|x| x.parse::<i64>().unwrap()).collect()).collect();
    
    l.iter().fold(0, |acc, v | if check_if_is_safe_with_tolerance(v) {acc+1} else {acc} ).to_string()
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


