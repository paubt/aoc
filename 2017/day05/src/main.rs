use std::fs;

fn part1(mut instruction_mem: Vec<i32>) -> u32 {
    let mut program_counter: i32 = 0;
    let mut cycle_counter = 0;
    let program_length: usize = instruction_mem.len();
    loop {
        if 0 <= program_counter && program_counter < program_length as i32 {
            cycle_counter += 1;
            instruction_mem[program_counter as usize] += 1;
            program_counter += instruction_mem[program_counter as usize] - 1;
        } else {
            return cycle_counter;
        }
    }
}

fn part2(mut instruction_mem: Vec<i32>) -> u32 {
    let mut program_counter: i32 = 0;
    let mut cycle_counter = 0;
    let program_length: usize = instruction_mem.len();
    loop {
        if 0 <= program_counter && program_counter < program_length as i32 {
            cycle_counter += 1;
            if instruction_mem[program_counter as usize] >= 3 {
                instruction_mem[program_counter as usize] -= 1;
                program_counter += instruction_mem[program_counter as usize] + 1;
            } else {
                instruction_mem[program_counter as usize] += 1;
                program_counter += instruction_mem[program_counter as usize] - 1;
            }
        } else {
            return cycle_counter;
        }
    }
}

fn main() {
    let input = fs::read_to_string("../data/day5/input.txt").unwrap();
    let mut instruction_mem: Vec<i32> = input.lines().fold(vec![], |mut acc, l| {
        acc.push(l.parse::<i32>().unwrap());
        acc
    });
    println!(
        "day 2 solution part 1: {} part 2: {}",
        part1(instruction_mem.clone()),
        part2(instruction_mem)
    );
}
