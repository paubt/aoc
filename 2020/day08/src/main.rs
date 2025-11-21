use std::fs;

#[derive(Debug, Copy, Clone)]
enum Command {
    Nop(i64),
    Acc(i64),
    Jnp(i64),
}
fn read_code(input: &str) -> Vec<Command> {
    input
        .lines()
        .map(|x| {
            let x = x.split(' ').collect::<Vec<&str>>();
            let amount = x[1].parse::<i64>().unwrap();
            match x[0] {
                "acc" => Command::Acc(amount),
                "nop" => Command::Nop(amount),
                "jmp" => Command::Jnp(amount),
                _ => panic!(),
            }
        })
        .collect()
}

fn execute_until_loop(code: Vec<Command>) -> (i64, i64) {
    let code_len = code.len() as i64;
    let mut code_with_flag: Vec<(Command, bool)> = code.into_iter().map(|c| (c, false)).collect();
    let mut pc: i64 = 0;
    let mut acc = 0;
    loop {
        let (c, v) = code_with_flag.get_mut(pc as usize).unwrap();
        if *v == true {
            break;
        } else {
            *v = true
        }
        match c {
            Command::Nop(_) => {
                pc += 1;
            }
            Command::Acc(a) => {
                acc += *a;
                pc += 1;
            }
            Command::Jnp(a) => {
                pc += *a;
                if pc == code_len {
                    return (acc, pc);
                }
                if pc < 0 {
                    panic!()
                }
            }
        }
    }
    (acc, pc)
}

fn part1(input: &str) {
    let code = read_code(input);

    let (acc, pc) = execute_until_loop(code);

    println!("Part 1: {}", acc)
}

fn part2(input: &str) {
    let og_code = read_code(input);

    for i in 0..og_code.len() {
        let mut new_code = og_code.clone();
        let c = new_code.get_mut(i).unwrap();
        let mut skip = false;
        match c {
            Command::Nop(x) => {
                *c = Command::Jnp(*x);
            }
            Command::Acc(x) => {
                skip = true;
            }
            Command::Jnp(x) => {
                *c = Command::Nop(*x);
            }
        }
        if !skip {
            let (acc, pc) = execute_until_loop(new_code);
            if pc == og_code.len() as i64 {
                println!("Part 2: {}", acc);
                return;
            }
        }
    }
    println!("Part 2: not found");
}

fn main() {
    let code = fs::read_to_string("../data/day08/input.txt").unwrap();
    part1(&code);
    part2(&code);
}
