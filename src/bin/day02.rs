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
    fn new() -> Self {
        Default::default()
    }
    fn execute_pt1(&mut self, comm: &Command) {
        match comm {
            Command::Fwd(i) => self.xpos += i,
            Command::Down(i) => self.depth += i,
            Command::Up(i) => self.depth -= i,
        }
    }
    fn execute_pt2(&mut self, comm: &Command) {
        match comm {
            Command::Fwd(i) => {
                self.xpos += i;
                self.depth += self.aim * i;
            }
            Command::Down(i) => {
                self.aim += i;
            }
            Command::Up(i) => {
                self.aim -= i;
            }
        }
    }
}

fn main() {
    let instrs: Vec<Command> = util::file_to_vec("input/day02.txt")
        .iter()
        .map(|x| Command::parse(x))
        .collect();
    {
        let mut sub = Submarine::new();
        println!("[Part 1] Initial state: {:?}", sub);
        for instr in instrs.iter() {
            sub.execute_pt1(instr);
        }
        println!("[Part 1] End state: {:?}", sub);
        println!("[Part 1] Answer: {}", sub.xpos * sub.depth)
    }
    {
        let mut sub = Submarine::new();
        println!("[Part 2] Initial state: {:?}", sub);
        for instr in instrs.iter() {
            sub.execute_pt2(instr);
        }
        println!("[Part 2] End state: {:?}", sub);
        println!("[Part 2] Answer: {}", sub.xpos * sub.depth)
    }
}
