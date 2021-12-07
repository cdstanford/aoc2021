/*
    Advent of Code 2021 Day 1
*/

use aoc2021::util;
use std::convert::TryInto;
use std::fmt::Debug;

/*
    Iterator over N-tuples
    Note: requires N >= 1
*/
struct TupleIter<T, I: Iterator<Item = T>, const N: usize> {
    prev: Vec<T>, // vector of strictly less than N items
    sub: I,
}
impl<T, I, const N: usize> Iterator for TupleIter<T, I, N>
where
    T: Copy + Debug,
    I: Iterator<Item = T>,
{
    type Item = [T; N];

    fn next(&mut self) -> Option<Self::Item> {
        while self.prev.len() + 1 < N {
            self.prev.push(self.sub.next()?);
        }
        debug_assert_eq!(self.prev.len() + 1, N);
        let mut result = Vec::new();
        for &i in &self.prev {
            result.push(i);
        }
        let new = self.sub.next()?;
        result.push(new);
        self.prev.push(new);
        self.prev.remove(0);
        debug_assert_eq!(result.len(), N);
        Some(result.try_into().unwrap())
    }
}

fn window<T, I, const N: usize>(iter: I) -> impl Iterator<Item = [T; N]>
where
    T: Copy + Debug,
    I: Iterator<Item = T>,
{
    TupleIter { prev: vec![], sub: iter }
}

fn main() {
    let depths: Vec<usize> = util::file_to_vec_parsed("input/day01.txt");
    let pt1 = window(depths.iter()).filter(|[d1, d2]| d1 < d2).count();
    println!("Part 1 answer: {}", pt1);
    let pt2 = window(depths.iter())
        .filter(|&[d1, d2, d3, d4]| d1 + d2 + d3 < d2 + d3 + d4)
        .count();
    println!("Part 2 answer: {}", pt2);
}
