use std::collections::{HashMap, HashSet};
use std::fs;

fn part1(mut memory: Vec<u32>) -> u32 {
    let mem_len: usize = memory.len();
    let mut red_cycle_counter: u32 = 0;
    let mut past_states: HashSet<Vec<u32>> = HashSet::new();
    past_states.insert(memory.clone());
    loop {
        red_cycle_counter += 1;
        let mut max_idx: usize = 0;
        let mut max_val: u32 = 0;
        for (idx, val) in memory.iter().enumerate() {
            if val > &max_val {
                max_idx = idx;
                max_val = *val;
            }
        }
        memory[max_idx] = 0;
        let base_value: u32 = max_val / mem_len as u32;
        memory.iter_mut().for_each(|ele| *ele += base_value);
        let mut left_over: i32 = max_val as i32 % mem_len as i32;
        let mut mem_iter = memory.iter_mut().skip(max_idx as usize + 1);
        'wl_loop: while let Some(v) = mem_iter.next() {
            if left_over > 0 {
                *v += 1;
                left_over -= 1;
            } else {
                break 'wl_loop;
            }
        }
        memory
            .iter_mut()
            .take(left_over as usize)
            .for_each(|ele| *ele += 1);

        // check if loop
        if !past_states.insert(memory.clone()) {
            return red_cycle_counter;
        }
    }
}

fn part2(mut memory: Vec<u32>) -> u32 {
    let mem_len: usize = memory.len();
    let mut red_cycle_counter: u32 = 0;
    let mut past_states: HashMap<Vec<u32>, u32> = HashMap::new();
    past_states.insert(memory.clone(), 0);
    loop {
        red_cycle_counter += 1;
        let mut max_idx: usize = 0;
        let mut max_val: u32 = 0;
        for (idx, val) in memory.iter().enumerate() {
            if val > &max_val {
                max_idx = idx;
                max_val = *val;
            }
        }
        memory[max_idx] = 0;
        let base_value: u32 = max_val / mem_len as u32;
        memory.iter_mut().for_each(|ele| *ele += base_value);
        let mut left_over: i32 = max_val as i32 % mem_len as i32;
        let mut mem_iter = memory.iter_mut().skip(max_idx as usize + 1);
        'wl_loop: while let Some(v) = mem_iter.next() {
            if left_over > 0 {
                *v += 1;
                left_over -= 1;
            } else {
                break 'wl_loop;
            }
        }
        memory
            .iter_mut()
            .take(left_over as usize)
            .for_each(|ele| *ele += 1);
        match past_states.insert(memory.clone(), red_cycle_counter) {
            Some(first_app) => return red_cycle_counter - first_app,
            None => continue,
        }
    }
}

fn main() {
    let instruction_mem: Vec<u32> = fs::read_to_string("../data/day6/input.txt")
        .unwrap()
        .split_whitespace()
        .fold(vec![], |mut acc, l| {
            acc.push(l.parse::<u32>().unwrap());
            acc
        });
    println!(
        "day 2 solution part 1: {} part 2: {}",
        part1(instruction_mem.clone()),
        part2(instruction_mem)
    );
}
