use crate::utils;
use std::fs;


pub fn solve_part1(input: &str) {
    let raw = fs::read_to_string(input).expect("could not read from file");
    let mut calories: Vec<usize> = raw
        .trim()
        .split("\n\n")
        .map(|s| s.lines().map(|s| s.parse::<usize>().unwrap()).sum())
        .collect();
    calories.sort();
    calories.reverse();
    println!("{:?}", calories.first().unwrap());
    println!("{:?}", calories.iter().take(3).sum::<usize>());
}
