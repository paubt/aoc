use std::fs;

fn part1(input: &str, right: usize, down: usize) -> usize {
    let mut x_pos = 0;
    let lines = input.lines().step_by(down).skip(1).collect::<Vec<&str>>();
    let width = lines[0].len();
    let mut count = 0;
    for l in lines {
        x_pos += right;
        x_pos = x_pos % width;
        if l.chars().nth(x_pos).unwrap() == '#' {
            count += 1;
        }
    }
    println!("part 1: {}", count);
    count
}

fn part2(input: &str) {
    let mut count = 1;
    for (r, d) in vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)] {
        count *= part1(input, r, d);
    }
    println!("part 2: {}", count);
}

fn main() {
    let code = fs::read_to_string("../data/day03/input.txt").unwrap();
    part1(&code, 3, 1);
    part2(&code);
}
