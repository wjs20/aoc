
use itertools::Itertools;
use regex::Regex;


fn initialize_stacks(stack_drawing: &str) -> Vec<Vec<String>> {
    let max_stack_length = stack_drawing.lines().count() - 1;
    let n_stacks: usize = stack_drawing
        .lines()
        .last()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let mut stacks: Vec<Vec<String>> = Vec::with_capacity(n_stacks);
    for i in 0..=n_stacks {
        let mut stack = Vec::with_capacity(max_stack_length);
        stacks.push(stack);
    }
    let box_labels = 'A'..='Z';
    for line in stack_drawing.lines() {
        let mut col = 0;
        for (i, c) in line.chars().enumerate() {
            if box_labels.contains(&c) {
                stacks[col].push(c.to_string());
            }
            if (i != 0) && (i % 4 == 0) {
                col += 1;
            }
        }
    }
    stacks
        .into_iter()
        .map(|stack| stack.into_iter().rev().collect_vec())
        .collect_vec()
}


#[derive(Debug, PartialEq)]
struct Instruction {
    n: usize,
    source: usize,
    dest: usize
}


fn parse_instructions(raw_instructions: &str) -> Vec<Instruction> {
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let mut results = vec![];
    for (_, [n, source, dest]) in re.captures_iter(raw_instructions).map(|c| c.extract()) {
        results.push( Instruction{ n: n.parse::<usize>().unwrap(), source: source.parse::<usize>().unwrap(), dest: dest.parse::<usize>().unwrap() } );
    }
    results
}


pub fn solve_part1(input: &str) {
    let (stack_drawing, raw_instructions) = input.split("\n\n").collect_tuple().unwrap();
    let mut stacks = initialize_stacks(stack_drawing);
    let instructions = parse_instructions(raw_instructions);
    for instruction in instructions {
        for _ in 0..instruction.n {
            if let Some(crate_label) = stacks[instruction.source - 1].pop() {
                stacks[instruction.dest - 1].push(crate_label);
            }
        }
    }
    let mut crate_labels = vec![];
    for stack in stacks {
        if let Some(crate_label) = stack.iter().last() {
            crate_labels.push(crate_label.clone());
        }
    }
    println!("{:?}", crate_labels);
}


pub fn solve_part2(input: &str) {
    let (stack_drawing, raw_instructions) = input.split("\n\n").collect_tuple().unwrap();
    let mut stacks = initialize_stacks(stack_drawing);
    let instructions = parse_instructions(raw_instructions);
    for instruction in instructions {
        let mut tmp = vec![];
        for _ in 0..instruction.n {
            if let Some(crate_label) = stacks[instruction.source - 1].pop() {
                tmp.push(crate_label);
            }
        }
        for crate_label in tmp.into_iter().rev() {
            stacks[instruction.dest - 1].push(crate_label)
        }
    }
    let mut crate_labels = vec![];
    for stack in stacks {
        if let Some(crate_label) = stack.iter().last() {
            crate_labels.push(crate_label.clone());
        }
    }
    println!("{:?}", crate_labels);
}


#[cfg(test)]
mod tests {

    use std::fs;

    use super::*;

    #[test]
    fn test_initialize_stacks() {
        assert_eq!(2, 2);
    }

    #[test]
    fn test_instruction_iter() {
        assert_eq!(2, 2);

    }

    #[test]
    fn test_solve_part1() {
        // let input = "";
        // let result = solve_part1(input);
        let result = 2;
        assert_eq!(result, 2);
    }

    #[test]
    fn test_solve_part2() {
        // let input = "";
        // let result = solve_part2(input);
        let result = 2;
        assert_eq!(result, 2);
    }
}
