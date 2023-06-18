use std::fs;
use std::str::FromStr;

use anyhow::Result;

#[derive(Debug, PartialEq, Eq)]
struct Present {
    l: u32,
    w: u32,
    h: u32,
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

        Ok(Present {
            l: l_from_str,
            h: h_from_str,
            w: w_from_str,
        })
    }
}

impl Present {
    fn compute_surface_areas(&self) -> Vec<u32> {
        let mut surface_areas = vec![];
        surface_areas.push(self.l * self.w);
        surface_areas.push(self.w * self.h);
        surface_areas.push(self.h * self.l);
        surface_areas
    }

    fn volume(&self) -> u32 {
        self.l * self.h * self.w
    }

    fn smallest_perimeter(&self) -> u32 {
        let side_lengths = vec![self.l, self.w, self.h];
        let mut sortable = side_lengths.to_vec();
        sortable.sort();
        sortable[0] * 2 + sortable[1] * 2
    }

    fn wrapping_paper_area(&self) -> u32 {
        let surface_areas = self.compute_surface_areas();
        let mut total_area: u32 = surface_areas.iter().map(|area| area * 2).sum();
        let smallest_area: u32 = *surface_areas.iter().min().unwrap();
        total_area += smallest_area;
        total_area
    }

    fn ribbon_length(&self) -> u32 {
        let surface_areas = self.compute_surface_areas();
        let perimeter = self.smallest_perimeter();
        let bow_length = self.volume();
        perimeter + bow_length
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

    #[test]
    fn test_present_ribbon_length() {
        let present = Present { l: 2, w: 3, h: 4 };
        assert_eq!(present.ribbon_length(), 34);

        let present = Present { l: 1, w: 1, h: 10 };
        assert_eq!(present.ribbon_length(), 14);
    }
}

pub fn solve_part1(input: &str) {
    let total_area: u32 = fs::read_to_string("input/2015/day02.txt")
        .expect("could not read file")
        .lines()
        .map(|line| line.parse::<Present>().expect("could not parse present"))
        .map(|present| present.wrapping_paper_area())
        .sum();

    println!("part 1: {:?}", total_area);
}

pub fn solve_part2(input: &str) {
    let total_length: u32 = fs::read_to_string("input/2015/day02.txt")
        .expect("could not read file")
        .lines()
        .map(|line| line.parse::<Present>().expect("could not parse present"))
        .map(|present| present.ribbon_length())
        .sum();

    println!("part 1: {:?}", total_length);
    // println!("part 2: {:?}", solution);
}
