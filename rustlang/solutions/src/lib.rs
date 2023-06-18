pub mod year2015;

// Define any common types or structs that may be used across days

// Define common utility functions
pub mod utils {
    use core::str::FromStr;
    use std::fmt::Debug;
    use std::fs;

    pub fn read_one_per_line<T: FromStr>(input_path: &str) -> anyhow::Result<Vec<T>>
    where
        <T as FromStr>::Err: Debug,
    {
        Ok(fs::read_to_string(input_path)
            .expect("could not read file")
            .lines()
            .map(|line| line.trim().parse::<T>().expect("unable to parse"))
            .collect())
    }

    pub fn read_chars(input_path: &str) -> anyhow::Result<Vec<char>> {
        Ok(fs::read_to_string(input_path)?.chars().collect())
    }

    pub fn sum_total(data: &[i32]) -> i32 {
        data.iter().sum()
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_read_one_per_line() {
//         assert_eq!(read_one_per_line(input_path), 4);
//     }
// }
