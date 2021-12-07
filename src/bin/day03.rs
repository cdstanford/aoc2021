/*
    AoC 2021 Day 3
*/

use aoc2021::util;
use std::cmp::Ordering;

// Number of indices in each binary string
const N: usize = 12;

/*
    Vote counter
    The main abstraction used by parts 1 and 2.
    Also a Criteria is an abstraction of a winning criteria for the vote
    counter, used for parts 1 and 2.
*/

#[derive(Debug, Default)]
struct VoteCounter {
    num_true: usize,
    num_false: usize,
}
impl VoteCounter {
    fn add(&mut self, b: bool) {
        if b {
            self.num_true += 1;
        } else {
            self.num_false += 1;
        }
    }
    fn get_winner(&self) -> Option<bool> {
        match self.num_true.cmp(&self.num_false) {
            Ordering::Less => Some(false),
            Ordering::Equal => None,
            Ordering::Greater => Some(true),
        }
    }
}

trait Criteria: Fn(&VoteCounter) -> bool {}
impl<F: Fn(&VoteCounter) -> bool> Criteria for F {}

fn gamma_criterion(c: &VoteCounter) -> bool {
    c.get_winner().unwrap()
}
fn eps_criterion(c: &VoteCounter) -> bool {
    !gamma_criterion(c)
}
fn o2_criterion(c: &VoteCounter) -> bool {
    c.get_winner().unwrap_or(true)
}
fn co2_criterion(c: &VoteCounter) -> bool {
    !o2_criterion(c)
}

/*
    Parsing
*/

fn parse_input(raw: &[String]) -> Vec<[bool; N]> {
    let mut result = Vec::new();
    for bstr in raw {
        debug_assert_eq!(bstr.chars().count(), N);
        let mut row = [false; N];
        for (i, ch) in bstr.chars().enumerate() {
            row[i] = util::char_to_bool(ch);
        }
        result.push(row);
    }
    result
}

fn bools_to_usize(input: &[bool]) -> usize {
    let mut result_str = String::new();
    for &b in input {
        result_str.push(util::bool_to_char(b));
    }
    usize::from_str_radix(&result_str, 2).unwrap()
}

/*
    Solutions

    - get_pt1 used for gamma and epsilon
    - get_pt2 used for co2 and o2 levels
*/

fn get_pt1<F: Criteria>(input: &[[bool; N]], f: F) -> usize {
    let mut counters: [VoteCounter; N] = Default::default();
    for row in input {
        for (i, &b) in row.iter().enumerate() {
            counters[i].add(b);
        }
    }
    let bools: Vec<bool> = counters.iter().map(|c| f(&c)).collect();
    bools_to_usize(&bools)
}

fn get_pt2<F: Criteria>(input: &[[bool; N]], f: F) -> usize {
    // Precondition
    debug_assert!(input.len() > 1);
    let mut contenders: Vec<[bool; N]> = input.to_vec();
    for i in 0..N {
        let mut cntr: VoteCounter = Default::default();
        for row in &contenders {
            cntr.add(row[i]);
        }

        let winner = f(&cntr);
        let mut new: Vec<[bool; N]> = Vec::new();
        for row in &contenders {
            if row[i] == winner {
                new.push(*row);
            }
        }
        contenders = new;

        if contenders.len() == 1 {
            return bools_to_usize(&contenders[0]);
        } else if contenders.is_empty() {
            panic!("No contenders left!");
        }
    }
    panic!("More than one contender left at the end!");
}

fn main() {
    let input = parse_input(&util::file_to_vec("input/day03.txt"));
    let gamma = get_pt1(&input, gamma_criterion);
    let eps = get_pt1(&input, eps_criterion);
    println!("Part 1 answer: {}", gamma * eps);
    let o2 = get_pt2(&input, o2_criterion);
    let co2 = get_pt2(&input, co2_criterion);
    println!("Part 2 answer: {}", o2 * co2);
}
