use itertools::iproduct;
use std::ops::Range;

use itertools::Itertools;

fn into_range(s: &str) -> Range<usize> {
    let (start, end) = s.split("-").map(|x| x.parse().unwrap()).collect_tuple().unwrap();
    Range { start, end: end + 1 }
}

fn is_completely_overlapping(r1: Range<usize>, r2: Range<usize>) -> bool {
    (
        (r1.start >= r2.start && r1.end <= r2.end) ||
        (r1.start <= r2.start && r1.end >= r2.end)
    )
}

fn is_partially_overlapping(r1: Range<usize>, r2: Range<usize>) -> bool {
    for i in r1 {
        if r2.contains(&i) {
            return true;
        }
    }
    false
}

pub fn solve_part1(input: &str) -> usize {

    let mut c = 0;
    for line in input.lines() {
       let (r1, r2) = line.split(",").map(|x| into_range(x)).collect_tuple().unwrap();
       if is_completely_overlapping(r1, r2) {
           c += 1;
       }
    }
    c
}

pub fn solve_part2(input: &str) -> usize {
    let mut c = 0;
    for line in input.lines() {
       let (r1, r2) = line.split(",").map(|x| into_range(x)).collect_tuple().unwrap();
       if is_partially_overlapping(r1, r2) {
           c += 1;
       }
    }
    c
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_into_range() {
        let input = "9-98";
        let result = into_range(input);
        assert_eq!(result, Range { start: 9, end: 99 })
    }

    #[test]
    fn test_is_completely_overlapping() {
        assert_eq!(is_completely_overlapping(2..4, 1..5), true);
    }

    #[test]
    fn test_is_partially_overlapping() {
        assert_eq!(is_partially_overlapping(2..4, 1..5), true);
        assert_eq!(is_partially_overlapping(2..4, 3..6), true);
        assert_eq!(is_partially_overlapping(2..4, 1..3), true);
        assert_eq!(is_partially_overlapping(2..4, 5..7), false);
        assert_eq!(is_partially_overlapping(2..4, 5..7), false);
    }

    #[test]
    fn test_solve_part1() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        let result = solve_part1(input);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_solve_part2() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        let result = solve_part2(input);
        assert_eq!(result, 4);
    }
}
