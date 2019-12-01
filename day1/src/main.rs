#![allow(dead_code)]
use std::fs::File;
use std::io::prelude::*;

fn part_one(mass: f32) -> f32 {
    (mass / 3.0 - 2.0).trunc()
}

fn part_two(mut mass: f32) -> f32 {
    let mut result = 0f32;
    loop {
        mass = part_one(mass);
        if mass as i32 <= 0 {
            break;
        }
        result += mass;
    }
    return result;
}

fn main() {
    let file = File::open("./input.txt").expect("file not found");
    let reader = std::io::BufReader::new(file);
    let bahan_bakar: i32 = reader
        .lines()
        .into_iter()
        .map(|e| part_two(e.unwrap().parse::<f32>().unwrap()) as i32)
        .sum();
    println!("{}", bahan_bakar);
}
