use std::fs;
use std::collections::HashMap;
use itertools::{iproduct, Itertools, Chunk};

fn split_pack(pack: &str) -> (&str, &str) {
    let half_pack = pack.len() / 2;
    pack.split_at(half_pack)
}

fn find_mispacked_item(compartment1: &str, compartment2: &str) -> char {
    for (i,j) in iproduct!(compartment1.chars(), compartment2.chars()) {
        if i == j {
            return i;
        }
    }
    panic!("No mispacked item found")

}

pub fn solve_part1(input: &str) {
    let packs = fs::read_to_string(input).expect("could not read from file");
    let priorities: HashMap<_, _> = ('a'..='z').chain('A'..='Z').zip(1..=52).collect();
    let (compartment1, compartment2) = split_pack("vJrwpWtwJgWrhcsFMMfFFhFp");
    let mispacked_item = find_mispacked_item(compartment1, compartment2);
    let item_priority = priorities.get(&mispacked_item);
    let result: usize = packs.lines().map(|pack|
        {
            let (compartment1, compartment2) = split_pack(pack);
            let mispacked_item = find_mispacked_item(compartment1, compartment2);
            priorities.get(&mispacked_item).unwrap()
        }
    ).sum();

    println!("{:?}", result);

}

fn identify_badge(group: &[&str]) -> char {
    let mut c: HashMap<char, usize> = HashMap::new();
    for &pack in group {
        let mut seen = String::new();
        for item in pack.chars() {
            if seen.contains(item) {
                continue;
            } else {
                seen.push(item)
            }
            let k = c.entry(item).or_insert(0);
            *k += 1;
            if *k == 3 {
                return item;
            }
        }
    }
    panic!();
}

pub fn solve_part2(input: &str) {
    let raw = fs::read_to_string(input).expect("could not read from file");
    let priorities: HashMap<_, _> = ('a'..='z').chain('A'..='Z').zip(1..=52).collect();
    let group_badges: Vec<char> = raw.lines()
        .chunks(3)
        .into_iter()
        .map(|group| identify_badge(&group.collect_vec()))
        .collect();

    let total_priority: usize = group_badges.iter().map(|item| priorities.get(item).unwrap()).sum();

    println!("{:?}", total_priority);
}
