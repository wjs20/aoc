use std::fs;
use std::str::FromStr;

use anyhow::Result;

#[derive(Debug, PartialEq, Eq)]
struct Present {
    l: u32,
    w: u32,
    h: u32
}

#[derive(Debug, PartialEq, Eq)]
struct ParsePresentError;

impl FromStr for Present {
    type Err = ParsePresentError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split("x").collect();

        let l_from_str = split[0].parse::<u32>().map_err(|_| ParsePresentError)?;
        let h_from_str = split[1].parse::<u32>().map_err(|_| ParsePresentError)?;
        let w_from_str = split[2].parse::<u32>().map_err(|_| ParsePresentError)?;

        Ok(Present { l: l_from_str, h: h_from_str, w: w_from_str })
    }
}

impl Present {
    fn wrapping_paper_area(&self) -> u32 {
        let mut wrapping_paper_area = 2*self.l*self.w + 2*self.w*self.h + 2*self.h*self.l;
        wrapping_paper_area += self.l*self.w; // slack
        wrapping_paper_area
    }
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_present_from_str() {
        let expected = Ok(Present { l: 1, w: 1, h: 1 });
        assert_eq!("1x1x1".parse::<Present>(), expected);
    }

    #[test]
    fn test_present_wrapping_paper_area() {
        let present = Present { l: 2, w: 3, h: 4 };
        assert_eq!(present.wrapping_paper_area(), 58);

        let present = Present { l: 1, w: 1, h: 10 };
        assert_eq!(present.wrapping_paper_area(), 43);
    }
}


pub fn solve_part1(input: &str) {
    let presents: Vec<Present> = fs::read_to_string("input/2015/day02.txt")
        .expect("could not read file")
        .lines()
        .map(|line| line.parse().expect("could not parse present"))
        .collect();
    println!("presents: {:?}", presents);

    let areas: u32 = presents.iter().map(|present| present.wrapping_paper_area()).sum();

    println!("part 1: {:?}", areas);
}

pub fn solve_part2(input: &str) {
    // println!("part 2: {:?}", solution);
}
