use std::collections::HashMap;
use std::fs;

#[derive(PartialEq, Debug, Copy, Clone)]
enum Cell {
    EmptySeat,
    Floor,
    OccupiedSeat,
}

fn part1(input: &str) {
    let initial: Vec<Vec<Cell>> = input
        .lines()
        .map(|x| {
            x.chars()
                .map(|x| match x {
                    '.' => Cell::Floor,
                    'L' => Cell::EmptySeat,
                    '#' => Cell::OccupiedSeat,
                    _ => panic!(),
                })
                .collect::<Vec<Cell>>()
        })
        .collect();

    let mut current_state = initial;
    loop {
        let mut next_state = current_state.clone();
        let mut something_changed = false;
        //let t  = current_state[1][1] == Cell::EmptySeat;
        for row_idx in 0..current_state.len() {
            for col_idx in 0..current_state[0].len() {
                // Count occupied neighbors.
                let mut occ_neig = 0;
                for row_offset in -1i64..2 {
                    for col_offset in -1i64..2 {
                        if row_offset == 0 && col_offset == 0 {
                            continue;
                        }
                        if row_idx as i64 + row_offset < 0
                            || row_idx as i64 + row_offset >= current_state.len() as i64
                        {
                            continue;
                        }
                        if col_idx as i64 + col_offset < 0
                            || col_idx as i64 + col_offset >= current_state[0].len() as i64
                        {
                            continue;
                        }

                        let r = (row_idx as i64 + row_offset) as usize;
                        let c = (col_idx as i64 + col_offset) as usize;

                        if current_state[r][c] == Cell::OccupiedSeat {
                            occ_neig += 1;
                        }
                    }
                }
                match current_state[row_idx][col_idx] {
                    Cell::EmptySeat => {
                        if occ_neig == 0 {
                            next_state[row_idx][col_idx] = Cell::OccupiedSeat;
                            something_changed = true
                        }
                    }
                    Cell::Floor => {}
                    Cell::OccupiedSeat => {
                        if occ_neig >= 4 {
                            next_state[row_idx][col_idx] = Cell::EmptySeat;
                            something_changed = true
                        }
                    }
                }
            }
        }
        current_state = next_state;
        if !something_changed {
            break;
        }
    }
    // Count occ seats.
    let cos = current_state.iter().fold(0, |acc, x| {
        acc + x.iter().fold(0, |acc, &y| {
            if y == Cell::OccupiedSeat {
                acc + 1
            } else {
                acc
            }
        })
    });
    println!("Part 1: {}", cos);
}

fn part2(input: &str) {
    let initial: HashMap<(i64, i64), Cell> =
        input
            .lines()
            .enumerate()
            .fold(HashMap::new(), |mut acc, (row, x)| {
                acc.extend(x.chars().enumerate().fold(
                    HashMap::new(),
                    |mut acc, (col, x)| match x {
                        '.' => {
                            acc.insert((row as i64, col as i64), Cell::Floor);
                            acc
                        }
                        'L' => {
                            acc.insert((row as i64, col as i64), Cell::EmptySeat);
                            acc
                        }
                        '#' => {
                            acc.insert((row as i64, col as i64), Cell::OccupiedSeat);
                            acc
                        }
                        _ => panic!(),
                    },
                ));
                acc
            });
    let mut current_state = initial;
    loop {
        let mut new_state = HashMap::new();
        let mut something_changed = false;

        fn count_neig(pos_r: i64, pos_c: i64, curr_s: &HashMap<(i64, i64), Cell>) -> i64 {
            let mut occ_neig = 0;
            // up

            fn check_dir<T, K>(
                row: T,
                col: K,
                rp: i64,
                cp: i64,
                cs: &HashMap<(i64, i64), Cell>,
            ) -> bool
            where
                T: IntoIterator<Item = i64>,
                K: IntoIterator<Item = i64>,
            {
                for (ro, co) in row.into_iter().zip(col) {
                    match cs.get(&(rp + ro, cp + co)) {
                        None => return false,
                        Some(v) => match v {
                            Cell::EmptySeat => return false,
                            Cell::Floor => {}
                            Cell::OccupiedSeat => return true,
                        },
                    }
                }
                false
            }
            // bot.
            if check_dir(
                (i64::MIN..0).rev(),
                [0].into_iter().cycle(),
                pos_r,
                pos_c,
                curr_s,
            ) {
                occ_neig += 1
            }
            // top.
            if check_dir(1.., [0].into_iter().cycle(), pos_r, pos_c, curr_s) {
                occ_neig += 1
            }
            // right.
            if check_dir([0].into_iter().cycle(), 1.., pos_r, pos_c, curr_s) {
                occ_neig += 1
            }
            // left.
            if check_dir(
                [0].into_iter().cycle(),
                (i64::MIN..0).rev(),
                pos_r,
                pos_c,
                curr_s,
            ) {
                occ_neig += 1
            }

            // TR.
            if check_dir(1.., 1.., pos_r, pos_c, curr_s) {
                occ_neig += 1
            }
            // TL.
            if check_dir(1.., (i64::MIN..0).rev(), pos_r, pos_c, curr_s) {
                occ_neig += 1
            }
            // BL
            if check_dir(
                (i64::MIN..0).rev(),
                (i64::MIN..0).rev(),
                pos_r,
                pos_c,
                curr_s,
            ) {
                occ_neig += 1
            }
            // BR
            if check_dir((i64::MIN..0).rev(), 1.., pos_r, pos_c, curr_s) {
                occ_neig += 1
            }
            occ_neig
        }

        for (&(r, c), v) in &current_state {
            match v {
                Cell::EmptySeat => {
                    let t = count_neig(r, c, &current_state);
                    if t == 0 {
                        new_state.insert((r, c), Cell::OccupiedSeat);
                        something_changed = true;
                    } else {
                        new_state.insert((r, c), Cell::EmptySeat);
                    }
                }
                Cell::Floor => {
                    new_state.insert((r, c), Cell::Floor);
                }
                Cell::OccupiedSeat => {
                    let t = count_neig(r, c, &current_state);
                    if t >= 5 {
                        new_state.insert((r, c), Cell::EmptySeat);
                        something_changed = true;
                    } else {
                        new_state.insert((r, c), Cell::OccupiedSeat);
                    }
                }
            }
        }
        current_state = new_state;
        if !something_changed {
            break;
        }
    }

    // Count occ seats.
    let cos = current_state.iter().fold(0, |acc, (_, &v)| {
        if v == Cell::OccupiedSeat {
            acc + 1
        } else {
            acc
        }
    });
    println!("Part 2: {}", cos);
}

fn main() {
    let code = fs::read_to_string("../data/day11/input.txt").unwrap();
    part1(&code);
    part2(&code);

}
