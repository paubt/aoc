use std::collections::HashSet;
use std::fs;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Seat {
    row: usize,
    col: usize,
}

fn read_list_to_seats(input: &str) -> Vec<Seat> {
    input.lines().map(|line| {
        if line.len() != 10 {panic!()};
        let to_number: String = line.chars().map(|c| match c {
            'B' | 'R' => '1',
            'F' | 'L' => '0',
            _ => panic!(),
        }).collect();
        let col_nr = usize::from_str_radix(&to_number[0..7], 2).unwrap();
        let row_nr = usize::from_str_radix(&to_number[7..10], 2).unwrap();
        Seat {row: col_nr, col: row_nr}
    }).collect::<Vec<Seat>>()
}
fn print_plan(seats: Vec<Seat>) {
    let seats: HashSet<Seat> = seats.iter().cloned().collect();
    for row in 0..128 {
        for col in 0..8 {
            if seats.get(&Seat { row, col }).is_some() {
                print!("X");
            } else {
                print!(" ");
            }
        }
        println!("     {}", row * 8);
    }

}
fn part1(input: &str) {
    let seats = read_list_to_seats(input);

    let max_id = seats.iter()
        .map(|seat| seat.row * 8 + seat.col)
        .max()
        .unwrap();
    println!("Part 1: {}", max_id);
}

fn part2(input: &str) {
    let seats = read_list_to_seats(input);
    print_plan(seats);
}

fn main() {
    let code = fs::read_to_string("../data/day05/input.txt").unwrap();
    part1(&code);
    part2(&code);
}
