use std::fs;
use md5::{ Md5, Digest };


fn hash_starts_with_n_0(s: &str, n: usize) -> bool {
    let tomatch = "0".repeat(n);
    let hash = Md5::digest(s);
    format!("{:x}", hash).starts_with(&tomatch)
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_hash_starts_with_n_0() {
        assert_eq!(hash_starts_with_n_0("abcdef609043", 5), true)
    }
}


pub fn solve_part1(input: &str) {

    let mut num = 0;
    loop {
        let input = format!("iwrupvqb{}", num);
        if hash_starts_with_n_0(&input, 5) {
            println!("part 1: {:?}", input);
            break;
        }
        num += 1;
    }
}

pub fn solve_part2(input: &str) {
    let mut num = 0;
    loop {
        let input = format!("iwrupvqb{}", num);
        if hash_starts_with_n_0(&input, 6) {
            println!("part 2: {:?}", input);
            break;
        }
        num += 1;
        if num % 1000 == 0 {
            println!("{}", num)
        }
    }
    println!("hello world")
}
