/*
    AoC 2021 Day 2
*/

use aoc2021::util;
use std::iter::FromIterator;

#[derive(Debug)]
enum Command {
    Fwd(usize),
    Down(usize),
    Up(usize),
    AimDown(usize),
    AimUp(usize),
}
impl Command {
    fn parse_pt1(raw: &str) -> Self {
        let (name, val): (String, usize) = util::split_pair_parsed(raw);
        match name.as_str() {
            "forward" => Self::Fwd(val),
            "down" => Self::Down(val),
            "up" => Self::Up(val),
            _ => panic!("Failed to parse command: {}", raw),
        }
    }
    fn parse_pt2(raw: &str) -> Self {
        let (name, val): (String, usize) = util::split_pair_parsed(raw);
        match name.as_str() {
            "forward" => Self::Fwd(val),
            "down" => Self::AimDown(val),
            "up" => Self::AimUp(val),
            _ => panic!("Failed to parse command: {}", raw),
        }
    }
}

#[derive(Debug)]
struct Submarine {
    xpos: usize,
    depth: usize,
    aim: usize,
}
impl Default for Submarine {
    fn default() -> Self {
        Submarine { xpos: 0, depth: 0, aim: 0 }
    }
}
impl Submarine {
    fn execute(&mut self, comm: &Command) {
        match comm {
            Command::Fwd(i) => {
                self.xpos += i;
                self.depth += self.aim * i;
            }
            Command::Down(i) => self.depth += i,
            Command::Up(i) => self.depth -= i,
            Command::AimDown(i) => self.aim += i,
            Command::AimUp(i) => self.aim -= i,
        }
    }
    fn answer(&self) -> usize {
        self.xpos * self.depth
    }
}
impl FromIterator<Command> for Submarine {
    fn from_iter<I: IntoIterator<Item = Command>>(iter: I) -> Self {
        let mut sub: Submarine = Default::default();
        println!("-> Submarine initial state: {:?}", sub);
        for comm in iter {
            sub.execute(&comm);
        }
        println!("-> Submarine final state: {:?}", sub);
        sub
    }
}

fn main() {
    let raw = util::file_to_vec("input/day02.txt");
    let pt1: Submarine = raw.iter().map(|x| Command::parse_pt1(x)).collect();
    println!("Part 1 answer: {}", pt1.answer());
    let pt2: Submarine = raw.iter().map(|x| Command::parse_pt2(x)).collect();
    println!("Part 2 answer: {}", pt2.answer());
}
