use std::collections::VecDeque;

#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
}

fn part1(input: u64) -> u64 {
    assert!(input != 0);
    // increasing step size search the circle ring number of the input number
    let mut search_momentum: u64 = 1;
    let mut search_direction: Direction = Direction::Down;
    let mut ring_num: u64 = 0;
    loop {
        let t = 4 * ((ring_num * ring_num) + ring_num) + 1;

        if t - (4 * 2 * ring_num) > input {
            if search_direction == Direction::Down {
                search_momentum = 2 * search_momentum;
            } else {
                search_momentum = 1;
                search_direction = Direction::Down;
            }
            ring_num -= search_momentum;
        } else if t < input {
            if search_direction == Direction::Up {
                search_momentum = 2 * search_momentum;
            } else {
                search_momentum = 1;
                search_direction = Direction::Up;
            }
            ring_num += search_momentum;
        } else {
            break;
        }
    }
    ring_num
        + ((((input - (4 * (((ring_num - 1) * (ring_num - 1)) + ring_num - 1) + 1) - 1)
            % (2 * ring_num)) as i64
            - ring_num as i64
            + 1)
        .abs() as u64)
}

fn add_x_to_rel_idx(x: u64, two: bool, (start, length): (usize, usize), deq: &mut VecDeque<u64>) {
    deq[1] += x;
    if two {
        deq[2] += x;
    }
    for ri in start..(start + length) {
        deq[ri] += x;
    }
}

macro_rules! check_exit_cond {
    ($queue:expr, $target:expr) => {
        if $queue.front().unwrap() >= $target {
            return *$queue.front().unwrap();
        } else {
            $queue.pop_front().unwrap();
        }
    };
}

fn part2(input: u64) -> u64 {
    // first number
    let mut deq = VecDeque::from([1; 8]);
    let mut ring_size = 2;
    let mut idx_ring = 6;
    // the first ring (r = 2)
    deq.append(&mut VecDeque::from(vec![0; (2 + ring_size) * 4]));
    // part1 -> first and last
    add_x_to_rel_idx(*deq.front().unwrap(), true, (idx_ring, 5), &mut deq);
    check_exit_cond!(deq, &input);
    idx_ring += 2;
    add_x_to_rel_idx(*deq.front().unwrap(), false, (idx_ring, 5), &mut deq);
    check_exit_cond!(deq, &input);
    // part 2 and 3
    for _ in 0..2 {
        idx_ring += 2;
        // first
        add_x_to_rel_idx(*deq.front().unwrap(), true, (idx_ring, 3), &mut deq);
        check_exit_cond!(deq, &input);
        // last
        add_x_to_rel_idx(*deq.front().unwrap(), false, (idx_ring, 5), &mut deq);
        check_exit_cond!(deq, &input);
    }
    // part 4
    idx_ring += 2;
    add_x_to_rel_idx(*deq.front().unwrap(), false, (idx_ring, 3), &mut deq);
    check_exit_cond!(deq, &input);

    add_x_to_rel_idx(*deq.front().unwrap(), true, (idx_ring, 3), &mut deq);
    check_exit_cond!(deq, &input);

    // loop for each ring
    loop {
        ring_size += 2;
        // allocate next ring with zeros
        deq.append(&mut VecDeque::from(vec![0; (2 + ring_size) * 4]));
        // for all four parts
        for part_idx in 0..4 {
            idx_ring += 2;
            // first
            if part_idx == 0 {
                add_x_to_rel_idx(*deq.front().unwrap(), false, (idx_ring - 2, 5), &mut deq);
                check_exit_cond!(deq, &input);
            } else {
                add_x_to_rel_idx(*deq.front().unwrap(), false, (idx_ring, 3), &mut deq);
                check_exit_cond!(deq, &input);
            }
            // calc how many middle parts there are (ring_size - 3)
            for _ in 0..ring_size - 3 {
                add_x_to_rel_idx(*deq.front().unwrap(), false, (idx_ring, 3), &mut deq);
                check_exit_cond!(deq, &input);
            }
            // pre last and last in one cause both differ only in fourth iteration
            if part_idx == 3 {
                add_x_to_rel_idx(*deq.front().unwrap(), false, (idx_ring, 3), &mut deq);
                check_exit_cond!(deq, &input);
                // last
                add_x_to_rel_idx(*deq.front().unwrap(), true, (idx_ring, 3), &mut deq);
                check_exit_cond!(deq, &input);
            } else {
                add_x_to_rel_idx(*deq.front().unwrap(), true, (idx_ring, 3), &mut deq);
                check_exit_cond!(deq, &input);
                // last
                add_x_to_rel_idx(*deq.front().unwrap(), false, (idx_ring, 5), &mut deq);
                check_exit_cond!(deq, &input);
            }
        }
    }
}

fn main() {
    println!(
        "day 2 solution part 1: {} part 2: {}",
        part1(277678),
        part2(277678)
    );
}
