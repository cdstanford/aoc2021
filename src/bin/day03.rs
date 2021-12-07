/*
    AoC 2021 Day 3
*/

use aoc2021::util;
use std::cmp::Ordering;

// Number of indices in each binary string
const N: usize = 12;

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

fn main() {
    let input = util::file_to_vec("input/day03.txt");
    let mut counters: [VoteCounter; N] = Default::default();
    for bstr in &input {
        debug_assert_eq!(bstr.chars().count(), N);
        for (i, ch) in bstr.chars().enumerate() {
            debug_assert!(ch == '0' || ch == '1');
            counters[i].add(ch == '1');
        }
    }
    let mut gamma = String::new();
    let mut eps = String::new();
    for cntr in counters.iter() {
        if cntr.get_winner().unwrap() {
            gamma.push('1');
            eps.push('0');
        } else {
            gamma.push('0');
            eps.push('1');
        }
    }
    let gamma = usize::from_str_radix(&gamma, 2).unwrap();
    let eps = usize::from_str_radix(&eps, 2).unwrap();
    println!("Part 1 answer: {}", gamma * eps);
}
