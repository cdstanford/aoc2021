/*
    AoC 2021 Day 2
*/

use aoc2021::util;

#[derive(Debug)]
enum Command {
    Fwd(usize),
    Down(usize),
    Up(usize),
}
impl Command {
    fn parse(raw: &str) -> Self {
        let (name, val): (String, usize) = util::split_pair_parsed(raw);
        match name.as_str() {
            "forward" => Self::Fwd(val),
            "down" => Self::Down(val),
            "up" => Self::Up(val),
            _ => panic!("Failed to parse command: {}", raw),
        }
    }
}

#[derive(Debug)]
struct Submarine {
    // Horizontal position
    x: usize,
    // Depth
    y: usize,
}
impl Default for Submarine {
    fn default() -> Self {
        Submarine { x: 0, y: 0 }
    }
}
impl Submarine {
    fn new() -> Self {
        Default::default()
    }
    fn execute(&mut self, comm: &Command) {
        match comm {
            Command::Fwd(i) => self.x += i,
            Command::Down(i) => self.y += i,
            Command::Up(i) => self.y -= i,
        }
    }
}

fn main() {
    let instrs: Vec<Command> = util::file_to_vec("input/day02.txt")
        .iter()
        .map(|x| Command::parse(x))
        .collect();
    let mut sub = Submarine::new();
    println!("Initial state: {:?}", sub);
    for instr in instrs.iter() {
        sub.execute(instr);
    }
    println!("End state: {:?}", sub);
    println!("Part 1 answer: {}", sub.x * sub.y)
}
