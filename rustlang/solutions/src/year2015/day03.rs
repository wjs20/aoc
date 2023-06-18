use std::collections::HashSet;
use std::fs;


fn decode_instruction(code: char) -> (i32, i32) {
    match code {
        '^' => (0, 1),
        '>' => (1, 0),
        'v' => (0, -1),
        '<' => (-1, 0),
        _ => panic!("unexpected char!"),
    }
}


#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct Position(i32, i32);

impl Position {
    fn origin() -> Position {
        Position { 0: 0, 1: 0 }
    }
}


#[derive(Debug)]
struct Sled {
    position: Position,
    log: Vec<Position>
}


impl Sled {
    fn new() -> Sled {
        let origin = Position::origin();
        let log: Vec<Position> = vec![];
        Sled { position: origin, log }
   }

    fn update_position(&mut self, instruction: (i32, i32)) {
        self.position = Position { 0: self.position.0 + instruction.0, 1: self.position.1 + instruction.1 };
    }

    fn update_log(&mut self, position: Position) {
        self.log.push(position);
    }

    fn visited(&self, position: &Position) -> bool {
        self.log.contains(position)
    }

    fn update(&mut self, code: char) {
        let instruction = decode_instruction(code);
        self.update_position(instruction);
        if !self.visited(&self.position) {
            self.update_log(self.position)
        }
    }

    fn make_deliveries(&mut self, codes: Vec<char>) {
        self.log.push(self.position);
        for c in codes {
            self.update(c);
        }
    }

    fn reset_log(&mut self) {
       self.log.clear();
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_make_deliveries_no_repeats() {
        let expected = 2;
        let input: Vec<char> = ">".chars().collect();
        let mut sled = Sled::new();
        sled.make_deliveries(input);
        let visited = sled.log.len();
        assert_eq!(visited, expected);
    }

    #[test]
    fn test_make_deliveries_with_repeats() {
        let expected = 4;
        let input: Vec<char> = "^>v<".chars().collect();
        let mut sled = Sled::new();
        sled.make_deliveries(input);
        let visited = sled.log.len();
        assert_eq!(visited, expected);
    }

    #[test]
    fn test_make_deliveries_with_many_repeats() {
        let expected = 2;
        let input: Vec<char> = "^v^v^v^v^v".chars().collect();
        let mut sled = Sled::new();
        sled.make_deliveries(input);
        let visited = sled.log.len();
        assert_eq!(visited, expected);
    }

}

pub fn solve_part1(input: &str) {
    let codes: Vec<char> = fs::read_to_string(input)
        .expect("could not read file")
        .chars()
        .filter_map(|c| {
            match c {
                '^' | '>' | 'v' | '<' => Some(c),
                _ => None,
            }
        }).collect();


    let mut sled = Sled::new();
    sled.make_deliveries(codes);
    let visited_count = sled.log.len();
    println!("part 1: {:?}", visited_count);
}

pub fn solve_part2(input: &str) {
    let codes: Vec<char> = fs::read_to_string(input)
        .expect("could not read file")
        .chars()
        .filter_map(|c| {
            match c {
                '^' | '>' | 'v' | '<' => Some(c),
                _ => None,
            }
        }).collect();

    let mut codes_sled: Vec<char> = vec![];
    let mut codes_robosled: Vec<char> = vec![];

    // separate instructions
    for (i, code) in codes.into_iter().enumerate() {
       if (i % 2) == 0 {
           codes_sled.push(code);
       } else {
           codes_robosled.push(code);
       }
    }

    let mut sled = Sled::new();
    let mut robosled = Sled::new();
    sled.make_deliveries(codes_sled);
    robosled.make_deliveries(codes_robosled);
    
    let visited_by_sled: HashSet<Position> = HashSet::from_iter(sled.log);
    let visited_by_robosled: HashSet<Position> = HashSet::from_iter(robosled.log);

    let visited: HashSet<_> = visited_by_sled.union(&visited_by_robosled).collect();

    let visited_count = visited.len();
    println!("part 2: {:?}", visited_count);
}
