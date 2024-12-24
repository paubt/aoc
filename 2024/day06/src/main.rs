use std::fs;
use std::fmt;

struct Zelle {
    obsacle: bool,
    new_obstacle: bool,
    visited_by_north: bool,
    visited_by_south: bool,
    visited_by_west: bool,
    visited_by_east: bool,
    turn: bool,
}

fn create_obstacle_cell() -> Zelle {
    Zelle { 
        obsacle: true, 
        new_obstacle: false, 
        visited_by_north: false, 
        visited_by_south: false, 
        visited_by_west: false, 
        visited_by_east: false, 
        turn: false 
    }
}

fn create_empty_cell() -> Zelle {
    Zelle { 
        obsacle: false, 
        new_obstacle: false, 
        visited_by_north: false, 
        visited_by_south: false, 
        visited_by_west: false, 
        visited_by_east: false, 
        turn: false 
    }
}

impl fmt::Display for Zelle {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::R esult` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        if self.new_obstacle {
            write!(f, "O")
        } else if self.obsacle {
            write!(f, "X")
        }else if self.turn || ((self.visited_by_north || self.visited_by_south) && 
                               (self.visited_by_east || self.visited_by_west)){
            write!(f, "+")
        } else if self.visited_by_north || self.visited_by_south  {
            write!(f, "|")
        }else if self.visited_by_west || self.visited_by_east  {
            write!(f, "—")
        } else {
            write!(f, ".")
        }
        
    }
}
#[derive(Debug)]
enum Direction {
   North,
   West,
   South,
   East 
}  
#[derive(Debug)]
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
                    '#' => create_obstacle_cell(),
                    '^' => {g = Some(Guard{x:ix, y:iy, heading: Direction::North}); 
                           create_empty_cell()}
                    '.' => create_empty_cell(),
                    _ => panic!("something other than # and . in input")
                })
                .collect::<Vec<Zelle>>())
        .collect::<Vec<Vec<Zelle>>>();
    let mut g = g.unwrap();
    let max_y = map.len() ;
    let max_x = map.first().unwrap().len() ;
    let mut i = 0;
    loop
    {
        // print!("Step {}\n", i);
        // i += 1;
        // dbg!(&g);
        // for col in &map {
        //     for ele in col {
        //         print!("{}",ele);
        //     }
        //     print!("\n");
        // }

        let mut new_x = g.x;
        let mut new_y = g.y;
        match g.heading {
            Direction::North =>  {if g.y == 0 {break;}  new_y -= 1},
            Direction::South =>  {if g.y == max_y - 1 {break;} new_y  += 1},
            Direction::West  =>  {if g.x == 0 {break;} new_x  -= 1},
            Direction::East  =>  {if g.x == max_x - 1 {break;} new_x += 1}
        }
        if map[new_y][new_x].obsacle == true{
            g.heading = turn_right(g.heading);
            map[g.y][g.x].turn = true;
        } else  {
            match g.heading {
                Direction::North => map[g.y][g.x].visited_by_north = true,
                Direction::West => map[g.y][g.x].visited_by_west = true,
                Direction::South => map[g.y][g.x].visited_by_south = true,
                Direction::East => map[g.y][g.x].visited_by_east = true,
            }
            
            g.y = new_y;
            g.x = new_x;
        }
    }
    match g.heading {
        Direction::North => map[g.y][g.x].visited_by_north = true,
        Direction::West => map[g.y][g.x].visited_by_west = true,
        Direction::South => map[g.y][g.x].visited_by_south = true,
        Direction::East => map[g.y][g.x].visited_by_east = true,
    }
    print!("Step {}\n", i);
    i += 1;
    dbg!(&g);
    for col in &map {
        for ele in col {
            print!("{}",ele);
        }
        print!("\n");
    }
    // get the amount of visited cells
    let sum_visited = map.iter()
        .fold(0, |acc, col| acc + col.iter()
            .fold(0, |acc, ele| 
                if ele.visited_by_north ||
                   ele.visited_by_south ||
                   ele.visited_by_west ||
                   ele.visited_by_east
                    {acc+1} else {acc}));
    sum_visited.to_string()
}

fn part2(input: String) -> String
{
    // let mut g: Option<Guard> = None;
    // let mut map = input.lines().enumerate()
    //     .map(|(iy,l)| l.chars().enumerate()
    //             .map(|(ix,c)| match c {
    //                 '#' => Zelle{obsacle:true, visited:false},
    //                 '^' => {g = Some(Guard{x:ix, y:iy, heading: Direction::North}); 
    //                        Zelle{obsacle:false, visited:false}}
    //                 '.' => Zelle{obsacle:false, visited:false},
    //                 _ => panic!("something other than # and . in input")
    //             })
    //             .collect::<Vec<Zelle>>())
    //     .collect::<Vec<Vec<Zelle>>>();
    // let mut g = g.unwrap();
    // let max_y = map.len() ;
    // let max_x = map.first().unwrap().len() ;
    // let mut i = 0;
    // loop
    // {
    //     // print!("Step {}\n", i);
    //     // i += 1;
    //     // dbg!(&g);
    //     // for col in &map {
    //     //     for ele in col {
    //     //         print!("{}",ele);
    //     //     }
    //     //     print!("\n");
    //     // }

    //     let mut new_x = g.x;
    //     let mut new_y = g.y;
    //     match g.heading {
    //         Direction::North =>  {if g.y == 0 {break;}  new_y -= 1},
    //         Direction::South =>  {if g.y == max_y - 1 {break;} new_y  += 1},
    //         Direction::West  =>  {if g.x == 0 {break;} new_x  -= 1},
    //         Direction::East  =>  {if g.x == max_x - 1 {break;} new_x += 1}
    //     }
    //     if map[new_y][new_x].obsacle == true{
    //         g.heading = turn_right(g.heading);
    //     }else  {
    //         map[g.y][g.x].visited = true;
    //         g.y = new_y;
    //         g.x = new_x;
    //     }
    // }
    // map[g.y][g.x].visited = true;
    // // print!("Step {}\n", i);
    // // i += 1;
    // // dbg!(&g);
    // // for col in &map {
    // //     for ele in col {
    // //         print!("{}",ele);
    // //     }
    // //     print!("\n");
    // // }
    // // get the amount of visited cells
    // let sum_visited = map.iter()
    //     .fold(0, |acc, col| acc + col.iter()
    //         .fold(0, |acc, ele| if ele.visited {acc+1} else {acc}));
    // sum_visited.to_string()    
    "todo".to_string()
}

fn read_in_input(path: &str) -> String 
{
    fs::read_to_string(path).unwrap()
}

fn main() 
{
    let input = read_in_input("./data/test.txt");
    println!(
        "day 3 solution part 1: {} part 2: {}",
        part1(input.clone()),
        part2(input)
    );
}

