use std::fs;

fn part1(input: &str) -> (usize, i64) {
    const WINDOW_SIZE: usize = 25;

    let v: Vec<i64> = input
        .split("\n")
        .map(|l| l.parse::<i64>().unwrap())
        .collect();

    'l0: for i in WINDOW_SIZE..v.len() {
        for u in i - WINDOW_SIZE..i - 1 {
            for w in u + 1..i {
                let a = v[u] + v[w];
                if a == v[i] {
                    continue 'l0;
                }
            }
        }
        return (i, v[i]);
    }
    panic!()
}

fn part2(input: &str) {
    let v: Vec<i64> = input
        .split("\n")
        .map(|l| l.parse::<i64>().unwrap())
        .collect();
    let (end, _) = part1(input);

    for i in 0..end {
        let mut s = v[i];
        if s == v[end] {
            print!("seq is {}-{}", i, i);
            return;
        }
        for j in i + 1..end {
            s += v[j];
            if s == v[end] {
                let ma = v.iter().skip(i).take(j - i).max().unwrap();
                let mi = v.iter().skip(i).take(j - i).min().unwrap();
                print!("seq is {}-{} resulting in target, this sum of min and max {}", i, j, ma + mi);
                return;
            }
        }
    }
    panic!()
}

fn main() {
    let code = fs::read_to_string("../data/day09/input.txt").unwrap();
    let (end, v) = part1(&code);
    println!("Part 1: found {} with value {}", end, v);
    part2(&code);
}
