use std::fs;

#[derive(Debug, Clone)]
struct Passport {
    byr: Option<String>, //(Birth Year)
    iyr: Option<String>, // (Issue Year)
    eyr: Option<String>, // (Expiration Year)
    hgt: Option<String>, // (Height)
    hcl: Option<String>, // (Hair Color)
    ecl: Option<String>, // (Eye Color)
    pid: Option<String>, //(Passport ID)
    cid: Option<String>, // (Country ID)
}

impl Passport {
    pub fn new() -> Self {
        Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        }
    }
    pub fn is_present(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    pub fn is_valid(&self) -> bool {
        if self.is_present() {
            let byr = self.byr.as_ref().unwrap().parse::<i64>().unwrap();
            if byr > 2002 || byr < 1920 {
                return false;
            }
            let iyr = self.iyr.as_ref().unwrap().parse::<i64>().unwrap();
            if iyr > 2020 || iyr < 2010 {
                return false;
            }
            let eyr = self.eyr.as_ref().unwrap().parse::<i64>().unwrap();
            if eyr < 2020 || eyr > 2030 {
                return false;
            }
            match self.hgt.as_ref().unwrap().len() {
                // cm
                5 => {
                    let ending = self.hgt.as_ref().unwrap().ends_with("cm");

                    let l = self
                        .hgt
                        .as_ref()
                        .unwrap()
                        .chars()
                        .take(3)
                        .collect::<String>()
                        .parse::<i64>()
                        .unwrap();
                    if !ending || l < 150 || l > 193 {
                        return false;
                    }
                }
                // in
                4 => {
                    let ending = self.hgt.as_ref().unwrap().ends_with("in");

                    let l = self
                        .hgt
                        .as_ref()
                        .unwrap()
                        .chars()
                        .take(2)
                        .collect::<String>()
                        .parse::<i64>()
                        .unwrap();
                    if !ending || l < 59 || l > 76 {
                        return false;
                    }
                }
                _ => {
                    return false;
                }
            }

            let mut hcl_iter = self.hcl.as_ref().unwrap().chars();
            if hcl_iter.next().unwrap() != '#' {
                return false;
            }
            let hcl_rest = hcl_iter.as_str();
            if hcl_rest.len() != 6 || !hcl_rest.chars().all(|c| c.is_ascii_hexdigit()) {
                return false;
            }

            let ecl = self.ecl.as_ref().unwrap();
            if !(ecl == "amb"
                || ecl == "blu"
                || ecl == "brn"
                || ecl == "gry"
                || ecl == "grn"
                || ecl == "hzl"
                || ecl == "oth")
            {
                return false;
            }

            let pid = self.pid.as_ref().unwrap();
            if pid.len() != 9 || pid.parse::<i64>().is_err() {
                return false;
            }

            true
        } else {
            return false;
        }
    }
}

fn readin_passports(input: &str) -> Vec<Passport> {
    input
        .split("\n\n")
        .map(|l| {
            let mut passport = Passport::new();
            for i in l.split(|c| c == ' ' || c == '\n') {
                let (key, val) = i.split_once(":").unwrap();
                match key {
                    "byr" => passport.byr = Some(val.parse().unwrap()),
                    "iyr" => passport.iyr = Some(val.parse().unwrap()),
                    "eyr" => passport.eyr = Some(val.parse().unwrap()),
                    "hgt" => passport.hgt = Some(val.parse().expect("hgt problem")),
                    "hcl" => passport.hcl = Some(val.parse().unwrap()),
                    "ecl" => passport.ecl = Some(val.to_string()),
                    "pid" => passport.pid = Some(val.parse().unwrap()),
                    "cid" => passport.cid = Some(val.parse().unwrap()),
                    &_ => panic!("Invalid key: {}", key),
                }
            }
            passport
        })
        .collect()
}

fn part1(input: &str) {
    let passports: Vec<Passport> = readin_passports(input);
    let valid_passports = passports.iter().filter(|p| p.is_present()).count();
    println!("part 1: {}", valid_passports);
}

fn part2(input: &str) {
    let passports: Vec<Passport> = readin_passports(input);
    let valid_passports: Vec<(Passport, bool)> = passports
        .iter()
        .map(|p| (p.clone(), p.is_valid()))
        .collect();
    println!(
        "part 1: {}",
        valid_passports.iter().filter(|(_, b)| *b).count()
    );
}

fn main() {
    let code = fs::read_to_string("../data/day04/input.txt").unwrap();
    part1(&code);
    part2(&code);
}
