use crate::utils;
use std::fs;
use regex::Regex;


fn n_vowels_ge(s: &str, n: usize) -> bool {
    let mut vowel_count = 0;
    for char in s.chars() {
        match char {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                vowel_count += 1;
                if vowel_count >= n {
                    return true
                }
            },
            _ => continue,
        }
    }
    false
}


fn contains_run(s: &str, n: usize) -> bool {
    let splitstr = s.chars().collect::<Vec<char>>();
    let strwindows: Vec<_> = splitstr.windows(n).collect();
    for win in &strwindows {
        let first = win[0];
        let all_same = win.iter().all(|&elem| elem == first);
        if all_same {
            return true
        }
    }
    false
}


fn contains_bad_strs(s: &str, bad_strs: &[&str]) -> bool {
    let mut found = vec![];
    for bad_str in bad_strs {
        found.push(s.contains(bad_str));
    }
    found.iter().any(|&x| x == true)
}


fn is_nice(s: &str) -> bool {
    let bad_strs = vec!["ab", "cd", "pq", "xy"];
    n_vowels_ge(&s, 3) && contains_run(&s, 2) && !contains_bad_strs(&s, &bad_strs)
}


fn count_occurences(s: &str, pat: Regex) -> usize {
    pat.find_iter(s).count()
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_n_vowels_gt() {
        assert_eq!(n_vowels_ge("aeiou", 3), true);
        assert_eq!(n_vowels_ge("aeith", 4), false);
    }

    #[test]
    fn test_contains_run() {
        assert_eq!(contains_run("aaaeiou", 3), true);
        assert_eq!(contains_run("aaeith", 3), false);
    }

    #[test]
    fn test_contains_bad_strs() {
        let bad_strs = vec!["ab", "cd", "pq", "xy"];
        assert_eq!(contains_bad_strs("aaaeiou", &bad_strs), false);
        assert_eq!(contains_bad_strs("aaeithxy", &bad_strs), true);
        assert_eq!(contains_bad_strs("aaeithpq", &bad_strs), true);
        assert_eq!(contains_bad_strs("aaeithcd", &bad_strs), true);
        assert_eq!(contains_bad_strs("aaeithab", &bad_strs), true);
    }

    #[test]
    fn test_is_nice() {
        assert_eq!(is_nice("ugknbfddgicrmopn"), true);
        assert_eq!(is_nice("aaa"), true);
        assert_eq!(is_nice("jchzalrnumimnmhp"), false);
        assert_eq!(is_nice("haegwjzuvuyypxyu"), false);
        assert_eq!(is_nice("dvszwmarrgswjxmb"), false);
    }

    #[test]
    fn test_count_occurences() {
        let s = String::from("aaa");
        let pat = Regex::new(r"\w").unwrap();
        assert_eq!(count_occurences(&s, pat), 3);
    }

    #[test]
    fn test_count_non_overlapping_pairs() {
        let s = String::from("xyxy abcdefgaa aaa");
        let pat = Regex::new(r"([a-zA-Z]{2}).*?\1").unwrap();
        assert_eq!(count_occurences(&s, pat), 2);
    }
}

pub fn solve_part1(input: &str) {
    let mut nice_count = 0;
    let total_length: Vec<_> = fs::read_to_string(input)
        .expect("could not read file")
        .lines()
        .map(|line| is_nice(line).then(|| { nice_count += 1; }))
        .collect();
    println!("part 1: {:?}", nice_count);
}

pub fn solve_part2(input: &str) {

    // println!("part 2: {:?}", solution);
}
