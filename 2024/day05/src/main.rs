use std::fs;
use std::collections::HashMap;


fn part1(input: String) -> String
{
    // Rules has for each Int all the Ints that have to go before in a vector.
    let mut rule_map: HashMap<i64,Vec<i64>> = HashMap::new();
    let mut manuals: Vec<Vec<i64>> = Vec::new();
    let mut rule_time = true;
    for l in input.lines()
    {
        if rule_time
        {
            if l == ""
            {
                rule_time = false;
                continue;
            }
            else 
            {
                let sl = l.split('|').collect::<Vec<&str>>();
                assert!(sl.len() == 2);
                match rule_map.get_mut(&sl[1].parse::<i64>().unwrap())
                {
                    None                   => {rule_map.insert(sl[1].parse::<i64>().unwrap(),vec!(sl[0].parse::<i64>().unwrap()));},
                    Some(v) => 
                    {
                        if !v.contains(&sl[0].parse::<i64>().unwrap())
                        {
                            v.insert(v.len(),sl[0].parse::<i64>().unwrap().clone());
                        }
                    }
                }
            }
        }
        else
        {
            manuals.insert(manuals.len(), l.split(',').map(|e| e.parse().unwrap()).collect());
        }
    }
    manuals.iter()
        .filter(|&manual| {
            assert!(manual.len() % 2 == 1);
            !check_if_illegal_manual(manual, &rule_map)
        })
        .map(|manual|manual[(manual.len()-1)/2])
        .sum::<i64>()
        .to_string()
}

fn check_if_illegal_manual(manual: &Vec<i64>, rule_map: &HashMap<i64,Vec<i64>>) -> bool
{
    for i in 0..manual.len() {
        match rule_map.get(&i64::try_from(manual[i]).unwrap()) {
            Some(nogos) => {
                for j in i+1..manual.len() {
                    if nogos.contains(&manual[j]) {
                        return true;
                    }
                }
            },
            None =>(),
        }
    }
    false
}

fn part2(input: String) -> String
{
    // Rules has for each Int all the Ints that have to go before in a vector.
    let mut rule_map: HashMap<i64,Vec<i64>> = HashMap::new();
    let mut manuals: Vec<Vec<i64>> = Vec::new();
    let mut rule_time = true;
    for l in input.lines()
    {
        if rule_time
        {
            if l == ""
            {
                rule_time = false;
                continue;
            }
            else 
            {
                let sl = l.split('|').collect::<Vec<&str>>();
                assert!(sl.len() == 2);
                match rule_map.get_mut(&sl[1].parse::<i64>().unwrap())
                {
                    None                   => {rule_map.insert(sl[1].parse::<i64>().unwrap(),vec!(sl[0].parse::<i64>().unwrap()));},
                    Some(v) => 
                    {
                        if !v.contains(&sl[0].parse::<i64>().unwrap())
                        {
                            v.insert(v.len(),sl[0].parse::<i64>().unwrap().clone());
                        }
                    }
                }
            }
        }
        else
        {
            manuals.insert(manuals.len(), l.split(',').map(|e| e.parse().unwrap()).collect());
        }
    }
    manuals.iter_mut()
        .filter(|manual| {
            assert!(manual.len() % 2 == 1);
            check_if_illegal_manual(manual, &rule_map)
        })
        .map(|manual|{
            loop {
                'all: for i in 0..manual.len() {
                    match rule_map.get(&i64::try_from(manual[i]).unwrap()) {
                        Some(nogos) => {
                            for j in i+1..manual.len() {
                                if nogos.contains(&manual[j]) {
                                    manual.swap(i, j);
                                    continue 'all;
                                }
                            }
                        },
                        None =>(),
                    }
                }
                if !check_if_illegal_manual(manual, &rule_map) {
                    break;
                }
            }
            manual.clone()
        })
        .map(|manual|manual[(manual.len()-1)/2])
        .sum::<i64>()
        .to_string()
}

fn read_in_input(path: &str) -> String 
{
    fs::read_to_string(path).unwrap()
}

fn main() 
{
    let input = read_in_input("./data/input.txt");
    println!(
        "day 3 solution part 1: {} part 2: {}",
        part1(input.clone()),
        part2(input)
    );
}

