use std::fs;
use itertools::iproduct;

const WORD: [char; 4]= ['X', 'M', 'A', 'S'];

const WORD_LEN: usize = WORD.len();

fn check_word_in_field_with_direction(
    field: &Vec<Vec<char>>,
    (row_size,col_size): (i64,i64),
    (row_idx,col_idx): (i64,i64),
    (row_dir,col_dir): (i64,i64))
    -> bool
{
    for i in 0..WORD_LEN
    {
        let r = row_idx+(row_dir*i64::try_from(i).unwrap());
        let c = col_idx+(col_dir*i64::try_from(i).unwrap());
        if r < 0 || r >= row_size || c < 0 || c >= col_size
        {
            return false;
        }
        let ur = usize::try_from(r).unwrap();
        let uc = usize::try_from(c).unwrap();
        if field[ur][uc] != WORD[i]
        {
            return false;
        }
    }
    true
}

fn part1(input: String) -> String
{
    let field: Vec<Vec<char>> = input.lines()
            .map(|l| l.chars().collect()).collect();
    
    let mut word_counter = 0;
    let row_size = i64::try_from(field.len()).unwrap();
    let col_size = i64::try_from(field.first().unwrap().len()).unwrap();
    for row_idx in 0..row_size
    {
        for col_idx in 0..col_size
        {
            word_counter += iproduct!(-1..2,-1..2).map(|(rd,cd)| {
                    if cd== 0 && rd == 0
                    {
                        return 0;
                    }
                    if check_word_in_field_with_direction(&field,
                        (row_size,col_size),
                        (row_idx,col_idx),
                        (rd,cd))
                    {
                        1
                    }
                    else 
                    {
                        0    
                    }
                }).sum::<i32>();
        }
    }
    word_counter.to_string()
}

fn check_x_mas_in_field_with_direction(
    field: &Vec<Vec<char>>,
    (row_idx,col_idx): (i64,i64))
    -> bool
{
    let ur = usize::try_from(row_idx).unwrap();
    let uc = usize::try_from(col_idx).unwrap();
    if field[ur][uc] == 'A'
    {
        if (field[ur-1][uc-1] == 'M' &&
            field[ur-1][uc+1] == 'M' &&
            field[ur+1][uc+1] == 'S' &&
            field[ur+1][uc-1] == 'S') 
            ||
           (field[ur-1][uc-1] == 'M' &&
            field[ur-1][uc+1] == 'S' &&
            field[ur+1][uc+1] == 'S' &&
            field[ur+1][uc-1] == 'M')
            ||
           (field[ur-1][uc-1] == 'S' &&
            field[ur-1][uc+1] == 'S' &&
            field[ur+1][uc+1] == 'M' &&
            field[ur+1][uc-1] == 'M')
            ||
           (field[ur-1][uc-1] == 'S' &&
            field[ur-1][uc+1] == 'M' &&
            field[ur+1][uc+1] == 'M' &&
            field[ur+1][uc-1] == 'S')
        {
            return true;
        }
    }
    false
}

fn part2(input: String) -> String
{
    let field: Vec<Vec<char>> = input.lines()
            .map(|l| l.chars().collect()).collect();
    let mut x_count = 0;
    let row_size = i64::try_from(field.len()).unwrap();
    let col_size = i64::try_from(field.first().unwrap().len()).unwrap();
    for row_idx in 1..row_size-1
    {
        for col_idx in 1..col_size-1
        {
            if check_x_mas_in_field_with_direction(&field,
                (row_idx,col_idx))
            {
                x_count += 1;
            }
        }
    }
    x_count.to_string()
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

