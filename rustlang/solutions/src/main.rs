use std::fs;
use solutions::year2022;

fn main() {

    println!("day05");
    let path = "input/2022/day05.txt";
    let input = fs::read_to_string(path).expect("could not read from file");
    year2022::day05::solve_part1(&input);
    // println!("{}", year2022::day04::solve_part2(&input));
    println!("--------------");
}
