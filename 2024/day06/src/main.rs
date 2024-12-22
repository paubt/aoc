use std::fs;

struct Zelle {
    obsacle: bool,
    visited: bool,
}

enum Direction {
   North,
   West,
   South,
   East 
}  

struct Guard {
    x: usize,
    y: usize,
    heading: Direction
}

fn turn_right (dir: Direction) -> Direction {
    match dir {
        Direction::North => Direction::East,
        Direction::West => Direction::North,
        Direction::South => Direction::West,
        Direction::East =>Direction::South,
    }
}

fn part1(input: String) -> String
{   
    let mut g: Option<Guard> = None;
    let mut map = input.lines().enumerate()
        .map(|(iy,l)| l.chars().enumerate()
                .map(|(ix,c)| match c {
                    '#' => Zelle{obsacle:true, visited:false},
                    '^' => {g = Some(Guard{x:ix, y:iy, heading: Direction::North}); 
                           Zelle{obsacle:false, visited:false}}
                    '.' => Zelle{obsacle:false, visited:false},
                    _ => panic!("something other than # and . in input")
                })
                .collect::<Vec<Zelle>>())
        .collect::<Vec<Vec<Zelle>>>();
    let mut g = g.unwrap();
    let max_y = map.len() ;
    let max_x = map.first().unwrap().len() ;
    
    loop
    {
        let mut new_x = g.x;
        let mut new_y = g.y;
        match g.heading {
            Direction::North =>  {if g.y == max_y - 1 {break;} new_y += 1},
            Direction::South =>  {if g.y == 0 {break;} new_y  -= 1},
            Direction::West  =>  {if g.x == max_x - 1 {break;} new_x += 1},
            Direction::East  =>  {if g.x == 0 {break;} new_x  -= 1}
        }
        if map[new_y][new_x].obsacle == true{
            g.heading = turn_right(g.heading);
        }else  {
            map[g.y][g.x].visited = true;
            g.y = new_y;
            g.x = new_x;
        }
    }
    
    "todo".to_string()
}

fn part2(input: String) -> String
{
    "todo".to_string()    
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

