/*
    AoC 2021 Day 4
*/

use aoc2021::util;
use std::collections::HashSet;
use std::convert::TryInto;

// Bingo board dimensions
const N: usize = 5;
// Number of bingo rows on the N x N board
const N_BINGO_ROWS: usize = 12;

#[derive(Debug, Clone)]
struct BingoLine {
    board_id: usize,
    remaining: HashSet<usize>,
}
impl BingoLine {
    fn new(board_id: usize, row: impl IntoIterator<Item = usize>) -> Self {
        let remaining: HashSet<usize> = row.into_iter().collect();
        debug_assert_eq!(remaining.len(), N);
        Self { board_id, remaining }
    }
    #[allow(dead_code)]
    fn remove(&mut self, i: usize) {
        self.remaining.remove(&i);
    }
    #[allow(dead_code)]
    fn is_won(&self) -> bool {
        self.remaining.is_empty()
    }
}

#[derive(Debug)]
struct BingoBoard {
    id: usize,
    board: [[usize; N]; N],
}
impl BingoBoard {
    fn get_bingo_lines(&self) -> [BingoLine; N_BINGO_ROWS] {
        let mut lines = Vec::new();
        // Rows
        for i in 0..N {
            let row = self.board[i].iter().copied();
            lines.push(BingoLine::new(self.id, row));
        }
        // Columns
        for i in 0..N {
            let col = self.board.iter().map(|x| x[i]);
            lines.push(BingoLine::new(self.id, col));
        }
        // Diagonals
        let mut diag1 = Vec::new();
        let mut diag2 = Vec::new();
        for i in 0..N {
            diag1.push(self.board[i][i]);
            diag2.push(self.board[i][N - 1 - i]);
        }
        lines.push(BingoLine::new(self.id, diag1));
        lines.push(BingoLine::new(self.id, diag2));
        debug_assert_eq!(lines.len(), N_BINGO_ROWS);
        lines.try_into().unwrap()
    }
}

fn parse_input(
    raw: &[String],
) -> (Vec<usize>, Vec<BingoBoard>, Vec<BingoLine>) {
    let mut raw_iter = raw.iter();
    let nums: Vec<usize> = util::split_by_parsed(raw_iter.next().unwrap(), ',');
    let mut boards: Vec<BingoBoard> = Vec::new();
    let mut lines: Vec<BingoLine> = Vec::new();

    let mut id = 0;
    while let Some(s) = raw_iter.next() {
        assert_eq!(s, "");
        let mut rows = Vec::new();
        for _ in 0..N {
            let row: [usize; 5] =
                util::split_array_parsed(raw_iter.next().unwrap());
            rows.push(row);
        }
        let board = BingoBoard { id, board: rows.try_into().unwrap() };
        for line in board.get_bingo_lines().iter() {
            lines.push(line.clone())
        }
        boards.push(board);
        id += 1;
    }
    (nums, boards, lines)
}

fn main() {
    let raw = util::file_to_vec("input/day04.txt");
    let (nums, boards, lines) = parse_input(&raw);
    print!("{:?} {:?} {:?}", nums, boards, lines);
}
