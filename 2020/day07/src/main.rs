use std::collections::{HashMap, HashSet, LinkedList};
use std::fs;

#[derive(Debug)]
struct Rule {
    outer_color: String,
    inner_color: Vec<(usize, String)>,
}

fn read_rule_set(input: &str) -> Vec<Rule> {
    let t = input
        .lines()
        .map(|l| {
            let it = l.split(' ');
            let oc = it.clone().take(2).collect::<Vec<&str>>().join(" ");
            let ics = it.skip(4).collect::<Vec<&str>>().chunks(4).fold(
                vec![],
                |mut acc: Vec<(usize, String)>, x| {
                    if x.len() != 4 {
                        acc
                    } else {
                        let c = x[1..3].join(" ");
                        acc.push((x[0].parse::<usize>().unwrap(), c));
                        acc
                    }
                },
            );
            Rule {
                outer_color: oc,
                inner_color: ics,
            }
        })
        .collect();
    t
}

fn part1(input: &str) {
    let rules = read_rule_set(input);

    let mut visited_bags: HashSet<String> = HashSet::new();
    let mut open_bags: LinkedList<String> = LinkedList::new();
    open_bags.push_back("shiny gold".to_string());
    while let Some(x) = open_bags.pop_front() {
        visited_bags.insert(x.to_string());
        let r = rules
            .iter()
            .filter(|r| r.inner_color.iter().find(|(_, c)| *c == x).is_some())
            .map(|r| r.outer_color.clone())
            .collect::<Vec<String>>();
        for c in r {
            if visited_bags.get(&c).is_none() {
                open_bags.push_back(c.clone());
            }
        }
    }
    println!("Part 1: {}", visited_bags.len() - 1)
}

fn part2(input: &str) {
    let mut open_rules = read_rule_set(input);
    let mut bags: HashMap<String, usize> = HashMap::new();
    while !open_rules.is_empty() {
        let pos = open_rules
            .iter()
            .position(|x| x.inner_color.iter().all(|(_, s)| bags.get(s).is_some()))
            .unwrap();
        let next = open_rules.remove(pos);
        let ownsum = 1 + next
            .inner_color
            .iter()
            .map(|(a, s)| a * bags.get(s).unwrap())
            .sum::<usize>();
        bags.insert(next.outer_color.to_string(), ownsum);
    }
    println!("Part 2: {}", bags.get("shiny gold").unwrap() - 1)
}

fn main() {
    let code = fs::read_to_string("../data/day07/input.txt").unwrap();
    part1(&code);
    part2(&code);
}
