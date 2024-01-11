# ChatGPT generated solutions


## 2022 3
### part 1
```rust
use std::fs;

fn item_priority(c: char) -> u32 {
    match c {
        'a'..='z' => (c as u32) - ('a' as u32) + 1,
        'A'..='Z' => (c as u32) - ('A' as u32) + 27,
        _ => 0,
    }
}

fn find_common_item(s: &str) -> char {
    let (first_half, second_half) = s.split_at(s.len() / 2);
    for c in first_half.chars() {
        if second_half.contains(c) {
            return c;
        }
    }
    ' '  // Placeholder if no common item is found.
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Failed to read the file");
    let mut total_priority = 0;

    for line in contents.lines() {
        let common_item = find_common_item(line);
        total_priority += item_priority(common_item);
    }

    println!("Total sum of priorities: {}", total_priority);
}
```
### part 2

```rust
use std::fs;

fn item_priority(c: char) -> u32 {
    match c {
        'a'..='z' => (c as u32) - ('a' as u32) + 1,
        'A'..='Z' => (c as u32) - ('A' as u32) + 27,
        _ => 0,
    }
}

fn find_common_badge(items: &[&str]) -> char {
    let mut common_items = items[0].chars().collect::<Vec<char>>();
    \\ common_items contains the first pack in the group
    \\ skip(1) only iterates over packs 2 and 3
    for &rucksack in items.iter().skip(1) {
        common_items.retain(|&c| rucksack.contains(c));
    }
    common_items[0]  // Assuming there's always one common badge.
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Failed to read the file");
    let lines: Vec<&str> = contents.lines().collect();
    let mut total_priority = 0;

    for chunk in lines.chunks(3) {
        let common_badge = find_common_badge(chunk);
        total_priority += item_priority(common_badge);
    }

    println!("Total sum of priorities: {}", total_priority);
}
```

## 2022 4
### part 1
```
struct Range {
    start: i32,
    end: i32,
}

impl Range {
    fn contains(&self, other: &Range) -> bool {
        self.start <= other.start && self.end >= other.end
    }
}

fn count_containing_pairs(input: &str) -> i32 {
    let mut count = 0;
    input
        .lines()
        .map(|line| {
            line.split(',').map(|range_str| {
                let mut parts = range_str.split('-').map(|n| n.parse::<i32>().unwrap());
                Range {
                    start: parts.next().unwrap(),
                    end: parts.next().unwrap(),
                }
            }).collect::<Vec<_>>()
        })
        .for_each(|ranges| {
            if ranges[0].contains(&ranges[1]) || ranges[1].contains(&ranges[0]) {
                count += 1;
            }
        });

    count
}

fn main() {
    let input = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8";
    let result = count_containing_pairs(input);
    println!("Number of containing pairs: {}", result);
}

```
*notes on for\_each()*
Using for\_each() instead of map() at the end of the iterator chain in Rust has specific benefits and purposes, especially in the context of your problem.
Purpose of for\_each(): The for\_each() method is used for executing a closure on each element of the iterator. It's typically used for its side effects, such as modifying external variables or performing I/O operations. In your case, it's being used to increment the count variable whenever a pair meets the condition.
Purpose of map(): On the other hand, map() is used for transforming each item of an iterator into a new form. It's a functional programming tool that applies a function to each element and collects the results into a new iterator. However, it's lazy, meaning it doesn't actually perform any operations until the results are needed (usually when collected into a container or iterated over).
Performance and Semantics: Using for\_each() is more semantically correct when you're not interested in the transformed results but only want to perform operations on each element. map() is less suitable here since you're not transforming the iterator's elements into a new form but merely counting specific cases. Using map() without collecting its results (like with .collect()) would be a misuse and won't have any effect due to its lazy nature.
Clarity and Intent: Using for\_each() clearly communicates the intent of the code to other developers: you're performing an action for each element, not transforming each element into something else. This makes the code more readable and maintainable.

```
impl Range {
    fn overlaps(&self, other: &Range) -> bool {
        self.start <= other.end && other.start <= self.end
    }
}

fn count_overlapping_pairs(input: &str) -> i32 {
    let mut count = 0;
    input
        .lines()
        .map(|line| {
            line.split(',').map(|range_str| {
                let mut parts = range_str.split('-').map(|n| n.parse::<i32>().unwrap());
                Range {
                    start: parts.next().unwrap(),
                    end: parts.next().unwrap(),
                }
            }).collect::<Vec<_>>()
        })
        .for_each(|ranges| {
            if ranges[0].overlaps(&ranges[1]) {
                count += 1;
            }
        });

    count
}
```
