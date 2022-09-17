use std::collections::{HashMap, HashSet};
use std::fs;

fn part1(input: &str) -> &str {
    let mut ref_once: HashSet<&str> = HashSet::new();
    let mut programs: Vec<Vec<&str>> = input
        .lines()
        .map(|l| {
            let mut v: Vec<&str> = l.split_whitespace().collect();
            if v.len() > 2 {
                v.remove(2);
                v.iter_mut()
                    .skip(2)
                    .rev()
                    .skip(1)
                    .for_each(|e| *e = &e[..e.len() - 1]);
            }
            v
        })
        .collect();
    let mut last_addition: &str = "tet";
    loop {
        if programs.len() <= 1 {
            if programs.len() == 0 {
                return last_addition;
            } else {
                return programs[0][0];
            }
        }
        programs.retain(|l| {
            if l.len() <= 2 {
                ref_once.insert(l[0]);
                false
            } else {
                // check if all child present
                let t = l.len() - 2;
                let k = l
                    .iter()
                    .skip(2)
                    .filter(|f| match ref_once.get(*f) {
                        Some(_) => true,
                        None => false,
                    })
                    .count();
                if t == k {
                    ref_once.insert(l[0]);
                    last_addition = l[0];
                    false
                } else {
                    true
                }
            }
        })
    }
}

fn part2(mut input: &str) {
    let mut ref_once: HashMap<&str, (u64, u64)> = HashMap::new();
    let mut programs: Vec<Vec<&str>> = input
        .lines()
        .map(|l| {
            let mut v: Vec<&str> = l.split_whitespace().collect();
            if v.len() > 2 {
                v.remove(2);
                v.iter_mut()
                    .skip(2)
                    .rev()
                    .skip(1)
                    .for_each(|e| *e = &e[..e.len() - 1]);
            }
            v
        })
        .collect();
    let mut found = false;
    'l_l: loop {
        if found == true {
            break;
        }
        programs.retain(|l| {
            if l.len() <= 2 {
                ref_once.insert(l[0], (l[1][1..l[1].len() - 1].parse::<u64>().unwrap(), 0));
                false
            } else {
                // check if all child present
                let t = l.len() - 2;
                let k = l
                    .iter()
                    .skip(2)
                    .filter(|f| match ref_once.get(*f) {
                        Some(_) => true,
                        None => false,
                    })
                    .count();
                if t == k {
                    // check if all children add up to same weight
                    let t: Vec<u64> = l
                        .iter()
                        .skip(2)
                        .map(|f| match ref_once.get(*f) {
                            Some((x, y)) => (x + y),
                            None => panic!(),
                        })
                        .collect();
                    let first = t[0];
                    if t.iter().max() == t.iter().min() {
                        let (x, y) = ref_once.get(l[2]).unwrap();
                        let child_w = (l.len() - 2) as u64 * (x + y);
                        ref_once.insert(
                            l[0],
                            (l[1][1..l[1].len() - 1].parse::<u64>().unwrap(), child_w),
                        );
                        false
                    } else {
                        let v: Vec<(u64, u64)> = l.iter().skip(2).fold(vec![], |mut acc, f| {
                            acc.push(*ref_once.get(f).unwrap());
                            acc
                        });
                        if v[0].0 + v[0].1 != v[1].0 + v[1].0 && v[0].0 + v[0].1 != v[2].0 + v[2].1
                        {
                            print!("part 2: {}", v[1].0 + v[1].1 - v[0].1);
                        } else {
                            if v[0].0 + v[0].1 == v[1].0 + v[1].1 {
                                print!("part 2: {}", v[0].0 + v[0].1 - v[2].1);
                            } else {
                                print!("part 2: {}", v[0].0 + v[0].1 - v[1].1);
                            }
                        }
                        found = true;
                        false
                    }
                } else {
                    true
                }
            }
        })
    }
}

fn main() {
    let input: String = fs::read_to_string("../data/day7/input.txt").unwrap();
    print!("day 2 solution part 1: {} ", part1(&input),);
    part2(&input);
}
