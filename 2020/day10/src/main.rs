use std::fs;

fn part1(input: &str) {
    let mut xs: Vec<i64> = input.lines().map(|x| x.parse::<i64>().unwrap()).collect();
    let build_in_adapter = *xs.iter().max().unwrap() + 3;
    let outlet = 0;
    xs.push(build_in_adapter);
    xs.push(outlet);
    xs.sort();
    let first_diff: Vec<i64> = xs.windows(2).map(|x| x[1] - x[0]).collect();
    if *first_diff.iter().max().unwrap() > 3 {
        panic!("first diff larger 3");
    }
    let count_1 = first_diff.iter().filter(|x| **x == 1).count();
    let count_3 = first_diff.iter().filter(|x| **x == 3).count();
    println!("Part 1: {}", count_1 * count_3);
}

fn part2(input: &str) {
    let mut xs: Vec<i64> = input.lines().map(|x| x.parse::<i64>().unwrap()).collect();
    let build_in_adapter = *xs.iter().max().unwrap() + 3;
    let outlet = 0;
    xs.push(build_in_adapter);
    xs.push(outlet);
    xs.sort();
    let first_diff: Vec<i64> = xs.windows(2).map(|x| x[1] - x[0]).collect();
    // Split into vectors at 3.
    let mut fd_idx = first_diff.iter();
    let mut res: Vec<Vec<i64>> = vec![];
    let mut cur = vec![];
    while let Some(x) = fd_idx.next() {
        if *x != 3 {
            cur.push(*x);
        } else {
            res.push(cur);
            cur = vec![];
        }
    }
    res.push(cur);
    let p: usize = res
        .iter()
        .map(|x| x.len())
        .filter(|x| *x > 0)
        .map(|x| match x {
            1 => 1,
            2 => 2,
            3 => 4,
            4 => 7,
            5 => 13,
            _ => panic!(),
        })
        .fold(1, |acc, x| acc * x);
    println!("Part 2: {}", p);
    if p != 28346956187648 {
        panic!();
    }
}

fn main() {
    let code = fs::read_to_string("../data/day10/input.txt").unwrap();
    part1(&code);
    part2(&code);
}
