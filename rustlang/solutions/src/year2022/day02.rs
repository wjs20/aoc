use crate::utils;
use std::fs;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseHandError;

impl FromStr for Hand {
    type Err = ParseHandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A"|"X" => Ok(Hand::Rock),
            "B"|"Y" => Ok(Hand::Paper),
            "C"|"Z" => Ok(Hand::Scissors),
            _ => Err(ParseHandError),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseOutcomeError;

impl FromStr for Outcome {
    type Err = ParseOutcomeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Outcome::Loss),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err(ParseOutcomeError),
        }
    }
}

#[derive(Debug)]
enum Outcome {
    Win,
    Loss,
    Draw,
}

fn choose_hand(opponent: &Hand, outcome: Outcome) -> Hand {
    match outcome {
        Outcome::Win => match opponent {
            Hand::Rock => Hand::Paper,
            Hand::Paper => Hand::Scissors,
            Hand::Scissors => Hand::Rock
        },
        Outcome::Loss => match opponent {
            Hand::Rock => Hand::Scissors,
            Hand::Paper => Hand::Rock,
            Hand::Scissors => Hand::Paper
        },
        Outcome::Draw => opponent.clone(),
    }
}

fn score_round(hands: (Hand, Hand)) -> usize {

    let outcome = if hands.0 == hands.1 {
        Outcome::Draw
    } else {
        match hands {
            (Hand::Scissors, Hand::Rock) => Outcome::Win,
            (Hand::Rock, Hand::Paper) => Outcome::Win,
            (Hand::Paper, Hand::Scissors) => Outcome::Win,
            _ => Outcome::Loss
        }
    };

    let outcome_score = match outcome {
        Outcome::Loss => 0,
        Outcome::Draw => 3,
        Outcome::Win => 6
    };

    let hand_score = match hands.1 {
        Hand::Rock => 1,
        Hand::Paper => 2,
        Hand::Scissors => 3
    };

    outcome_score + hand_score
}

pub fn solve_part1(input: &str) {
    let raw = fs::read_to_string(input).expect("could not read from file");
    let hands: Vec<_> = raw
        .lines()
        .map(|r| r.split_once(" ").unwrap())
        .map(|(o, m)| (o.parse::<Hand>().unwrap(), m.parse::<Hand>().unwrap()))
        .collect();
    let scores: usize = hands.into_iter().map(|x| score_round(x)).sum();
    // println!("{:?}", scores)
}

pub fn solve_part2(input: &str) {
    let raw = fs::read_to_string(input).expect("could not read from file");
    let hands: Vec<_> = raw
        .lines()
        .map(|r| r.split_once(" ").unwrap())
        .map(|(h, o)| (h.parse::<Hand>().unwrap(), o.parse::<Outcome>().unwrap()))
        .collect();

    let mut score = 0;
    for (opponent, outcome) in hands {
        let hand = choose_hand(&opponent, outcome);
        score += score_round((opponent, hand));
    }

    println!("{:?}", score)
}
