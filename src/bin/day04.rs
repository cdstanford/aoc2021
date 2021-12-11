/*
    AoC 2021 Day 4
*/

use aoc2021::util;
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;

/// Bingo board dimensions
const N: usize = 5;
/// Number of bingo rows on the N x N board
const N_BINGO_ROWS: usize = 12;

#[derive(Debug, Clone, Copy)]
struct BoardId(usize);
impl BoardId {
    fn inc(&mut self) {
        self.0 += 1;
    }
}

#[derive(Debug, Clone)]
struct BingoLine {
    board_id: BoardId,
    remaining: HashSet<usize>,
}
impl BingoLine {
    fn new(board_id: BoardId, row: impl IntoIterator<Item = usize>) -> Self {
        let remaining: HashSet<usize> = row.into_iter().collect();
        debug_assert_eq!(remaining.len(), N);
        Self { board_id, remaining }
    }
    fn remove(&mut self, i: usize) {
        self.remaining.remove(&i);
    }
    fn is_won(&self) -> bool {
        self.remaining.is_empty()
    }
}

#[derive(Debug)]
struct BingoBoard {
    id: BoardId,
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

#[derive(Debug, Default)]
struct BingoGame {
    boards: Vec<BingoBoard>,
    lines: Vec<BingoLine>,
    // Map from a number to all lines including that number
    lines_including: HashMap<usize, Vec<usize>>,
    // Any winning board numbers
    winners: Vec<BoardId>,
    // Seen numbers
    seen: HashSet<usize>,
}
impl BingoGame {
    fn new() -> Self {
        Default::default()
    }
    /// Add a new N x N bingo board to the game
    fn add_board(&mut self, board: BingoBoard) {
        // Check that ID is as expected
        debug_assert_eq!(board.id.0, self.boards.len());

        // Add lines on the board
        for line in board.get_bingo_lines().iter() {
            let line_no = self.lines.len();
            self.lines.push(line.clone());
            for &num in line.remaining.iter() {
                self.lines_including
                    .entry(num)
                    .or_insert_with(Vec::new)
                    .push(line_no);
            }
        }

        self.boards.push(board);
    }
    /// Call a number, return winner if any and winning score
    fn call_num(&mut self, num: usize) -> Option<(BoardId, usize)> {
        self.seen.insert(num);
        let line_nos = self.lines_including.get(&num).unwrap();
        for &line_no in line_nos {
            self.lines[line_no].remove(num);
            if self.lines[line_no].is_won() {
                self.winners.push(self.lines[line_no].board_id)
            }
        }
        if self.winners.is_empty() {
            None
        } else {
            debug_assert_eq!(self.winners.len(), 1);
            let winner = self.winners[0];
            let score = num * self.get_sum(winner);
            Some((winner, score))
        }
    }
    /// Get board sum
    fn get_sum(&self, board_id: BoardId) -> usize {
        let mut sum = 0;
        for i in 0..N {
            for j in 0..N {
                let val = self.boards[board_id.0].board[i][j];
                if !self.seen.contains(&val) {
                    sum += val;
                }
            }
        }
        sum
    }
}

fn parse_input(raw: &[String]) -> (BingoGame, Vec<usize>) {
    let mut raw_iter = raw.iter();
    let nums: Vec<usize> = util::split_by_parsed(raw_iter.next().unwrap(), ',');
    let mut game = BingoGame::new();

    let mut id = BoardId(0);
    while let Some(s) = raw_iter.next() {
        assert_eq!(s, "");
        let mut rows = Vec::new();
        for _ in 0..N {
            let row: [usize; 5] =
                util::split_array_parsed(raw_iter.next().unwrap());
            rows.push(row);
        }
        let board = BingoBoard { id, board: rows.try_into().unwrap() };
        game.add_board(board);
        id.inc();
    }
    (game, nums)
}

fn main() {
    let raw = util::file_to_vec("input/day04.txt");
    let (mut game, nums) = parse_input(&raw);
    // print!("{:?} {:?}", game, nums);

    // Part 1
    for &num in &nums {
        if let Some((winner, score)) = game.call_num(num) {
            println!("Winning board #: {}", winner.0);
            println!("Board score (part 1 answer): {}", score);
            break;
        }
    }
}
