use std::fs;

fn part1(input: &str) {
    let a = input
        .lines()
        .filter(|l| {
            let mut i = l.split(' ');
            let b = i
                .next()
                .unwrap()
                .split('-')
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            let (lower, upper) = (b[0], b[1]);
            let target = i.next().unwrap().chars().nth(0).unwrap();
            let occurrence: i64 = i.next().unwrap().chars().filter(|c| *c == target).count() as i64;
            occurrence <= upper && occurrence >= lower
        })
        .count();
    println!("part 1: {}", a);
}

fn part2(input: &str) {
    let a = input
        .lines()
        .filter(|l| {
            let mut i = l.split(' ');
            let b = i
                .next()
                .unwrap()
                .split('-')
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            let (p1, p2) = (b[0], b[1]);
            let target = i.next().unwrap().chars().nth(0).unwrap();
            //let occ1:bool = i.next().unwrap().chars().filter(|c| *c == target).count() as i64;
            let password = i.next().unwrap();
            let occ1 = password.clone().chars().nth(p1 - 1).unwrap();
            let occ2 = password.clone().chars().nth(p2 - 1).unwrap();
            (occ1 == target && occ2 != target) || (occ1 != target && occ2 == target)
        })
        .count();
    println!("part 2: {}", a);
}

fn main() {
    let code = fs::read_to_string("../data/day02/input.txt").unwrap();
    part1(&code);
    part2(&code);
}
