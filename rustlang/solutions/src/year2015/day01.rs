use std::fs;
use crate::utils;

fn decode(codes: &str) -> Vec<i32> {
    codes
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        })
    .collect()
}

fn find_basement_char_position(directions: &[i32]) -> Option<usize> {
    let mut position = 0;
    for (i, d) in directions.iter().enumerate() {
        position += d;
        if (position) < 0 {
            return Some(i);
        }
    }
    return None;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_get_directions() {
        let test_string1 = String::from("(())");
        assert_eq!(decode(&test_string1), vec![1, 1, -1, -1]);
    }

    #[test]
    fn test_sum_directions() {
        let test_directions = vec![1, 1, -1, -1];
        assert_eq!(utils::sum_total(&test_directions), 0);
    }

    #[test]
    fn test_find_basement_char_positions() {
        let test_directions = vec![1, -1, -1, -1];
        assert_eq!(find_basement_char_position(&test_directions), Some(2));
    }

    #[test]
    fn test_find_basement_char_positions_not_found() {
        let test_directions = vec![1, 1, 1, -1];
        assert_eq!(find_basement_char_position(&test_directions), None);
    }
}

pub fn solve_part1(input: &str) {
    let codes = fs::read_to_string(input).expect("could not read from file");
    let directions = decode(&codes);
    let net_movement = utils::sum_total(&directions);
    println!("part 1: {:?}", net_movement);
}

pub fn solve_part2(input: &str) {
    let codes = fs::read_to_string(input).expect("could not read from file");
    let directions = decode(&codes);
    let first_basement_position = find_basement_char_position(&directions).unwrap();
    println!("part 2: {:?}", first_basement_position);
}
