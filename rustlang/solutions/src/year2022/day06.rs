use itertools::Itertools;
use std::collections::HashSet;

pub fn solve_part1(input: &str) {
    let mut i = 0;
    let mut seen: Vec<char> = vec![];
    for (i, w) in input.chars().collect_vec().windows(4).enumerate() {
        let mut set = HashSet::new();

        for &c in w {
            set.insert(c);
        }

        if set.len() == 4 {
            println!("{:?}", w);
            println!("{:?}", i + 4);
            break;
        }
    }
}

pub fn solve_part2(input: &str) {
    let mut i = 0;
    let mut seen: Vec<char> = vec![];
    let message_len = 14;
    for (i, w) in input.chars().collect_vec().windows(message_len).enumerate() {
        let mut set = HashSet::new();

        for &c in w {
            set.insert(c);
        }

        if set.len() == message_len {
            println!("{:?}", w);
            println!("{:?}", i + message_len);
            break;
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_solve_part1() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(solve_part1(input), 5);
    }

    #[test]
    fn test_solve_part2() {
        // let input = "";
        // let result = solve_part2(input);
        let result = 2;
        assert_eq!(result, 2);
    }
}
