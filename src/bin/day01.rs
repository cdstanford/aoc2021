/*
    Advent of Code 2021 Day 1
*/

use aoc2021::util;
use std::convert::TryInto;
use std::fmt::Debug;

/*
    Windowing abstraction

    A sliding window is an iterator over all adjacent N-tuples in a given
    input iterator.

    Note: requires N >= 1
*/

struct SlidingWindow<I: Iterator, const N: usize> {
    // N: window size
    // vector of the last (N-1) items seen (if any)
    prev: Vec<I::Item>,
    // sub-iterator the window is over
    sub: I,
}
impl<I: Iterator, const N: usize> Iterator for SlidingWindow<I, N>
where
    I: Iterator,
    <I as Iterator>::Item: Clone + Debug,
{
    type Item = [I::Item; N];

    fn next(&mut self) -> Option<Self::Item> {
        while self.prev.len() + 1 < N {
            self.prev.push(self.sub.next()?);
        }
        debug_assert_eq!(self.prev.len() + 1, N);
        let mut result = Vec::new();
        for i in &self.prev {
            result.push(i.clone());
        }
        let new = self.sub.next()?;
        result.push(new.clone());
        self.prev.push(new);
        self.prev.remove(0);
        debug_assert_eq!(result.len(), N);
        Some(result.try_into().unwrap())
    }
}

// Extend Iterator trait to allow windowing with .window() syntax
trait Window: Iterator + Sized {
    fn window<const N: usize>(self) -> SlidingWindow<Self, N> {
        SlidingWindow { prev: vec![], sub: self }
    }
}
impl<I: Iterator> Window for I {}

/*
    Solutions
*/

fn pt1_cond(&[d1, d2]: &[&usize; 2]) -> bool {
    d1 < d2
}

fn pt2_cond(&[d1, d2, d3, d4]: &[&usize; 4]) -> bool {
    d1 + d2 + d3 < d2 + d3 + d4
}

fn main() {
    let depths: Vec<usize> = util::file_to_vec_parsed("input/day01.txt");
    println!(
        "Part 1 answer: {}",
        depths.iter().window().filter(pt1_cond).count(),
    );
    println!(
        "Part 2 answer: {}",
        depths.iter().window().filter(pt2_cond).count(),
    );
}
