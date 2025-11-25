use nalgebra::{Rotation2, Vector2, Vector3};
use std::f64::consts::PI;
use std::fs;

fn part1(input: &str) {
    let t = input
        .lines()
        .fold(Vector3::new(0, 0, 90), |mut acc: Vector3<i64>, c| {
            let dir = c.chars().next().unwrap();
            let quant = c
                .chars()
                .skip(1)
                .collect::<String>()
                .parse::<i64>()
                .unwrap();
            match dir {
                'N' => acc.y += quant,
                'S' => acc.y -= quant,
                'W' => acc.x += quant,
                'E' => acc.x -= quant,

                'L' => acc.z = (acc.z - quant + 360) % 360,
                'R' => acc.z = (acc.z + quant + 360) % 360,
                'F' => match acc.z {
                    270 => acc.x += quant,
                    180 => acc.y -= quant,
                    90 => acc.x -= quant,
                    0 => acc.y += quant,
                    _ => panic!(),
                },
                _ => panic!(),
            }
            acc
        });
    println!("Part 1: {}", t.x.abs() + t.y.abs());
}

fn conv_Vi64_Vf64(inp: Vector2<i64>) -> Vector2<f64> {
    Vector2::new(inp.x as f64, inp.y as f64)
}
fn conv_Vf64_Vi64(inp: Vector2<f64>) -> Vector2<i64> {
    Vector2::new(inp.x.round() as i64, inp.y.round() as i64)
}

fn part2(input: &str) {
    let (pos, _) = input.lines().fold(
        (Vector2::new(0, 0), Vector2::new(10, 1)),
        |mut pos_way: (Vector2<i64>, Vector2<i64>), c| {
            let (mut pos, mut way) = pos_way;
            let dir = c.chars().next().unwrap();
            let quant = c
                .chars()
                .skip(1)
                .collect::<String>()
                .parse::<i64>()
                .unwrap();
            match dir {
                'N' => way.y += quant,
                'S' => way.y -= quant,
                'W' => way.x -= quant,
                'E' => way.x += quant,

                'L' => {
                    let m = Rotation2::new((quant as f64 * PI) / 180.);
                    way = conv_Vf64_Vi64(*m.matrix() * conv_Vi64_Vf64(way))
                }
                'R' => {
                    let m = Rotation2::new((-quant as f64 * PI) / 180.);
                    way = conv_Vf64_Vi64(*m.matrix() * conv_Vi64_Vf64(way))
                }
                'F' => {
                    let m: Vector2<i64> = way * quant;
                    pos = pos + m;
                }
                _ => panic!(),
            }
            (pos, way)
        },
    );
    println!("Part 2: {}", pos.x.abs() + pos.y.abs());
}

fn main() {
    let code = fs::read_to_string("../data/day12/input.txt").unwrap();
    part1(&code);
    part2(&code);
}
