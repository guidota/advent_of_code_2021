use std::{
    cmp::Ordering,
    fmt::{Display, Formatter},
};

mod part_1;
mod part_2;

use part_1::find_winner_board;
use part_2::find_last_winner_board;
use utils::*;
static FILE_NAME: &str = "day_4/resources/input.txt";

struct BingoInput {
    numbers: Vec<usize>,
    boards: Vec<Board<InitialState>>,
}

impl BingoInput {
    fn parse(input: &mut Input) -> Self {
        let numbers = input
            .lines
            .next()
            .expect("No numbers")
            .expect("No numbers")
            .split(',')
            .map(|s| s.parse::<usize>().expect("Invalid number"))
            .collect::<Vec<usize>>();

        let boards = Self::generate_boards(input);
        Self { numbers, boards }
    }

    fn generate_boards(input: &mut Input) -> Vec<Board<InitialState>> {
        let mut boards = Vec::new();
        loop {
            let empty_line = input.next();
            if empty_line.is_none() {
                break;
            }
            let board = Board::parse(input);
            boards.push(board);
        }
        boards
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Cell {
    number: usize,
    marked: bool,
}

impl Cell {
    fn new(number: usize) -> Self {
        Self {
            number,
            marked: false,
        }
    }

    fn update(&mut self, number: usize) {
        if self.number == number {
            self.marked = true;
        }
    }
}

#[derive(Debug)]
pub struct Board<BoardState> {
    grid: Vec<Vec<Cell>>,
    state: BoardState,
}

struct InitialState {}

pub struct FinalState {
    round: usize,
    final_number: usize,
}

impl Display for Board<FinalState> {
    fn fmt(&self, _: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.grid {
            for cell in row {
                print!("{}-{} | ", cell.number, if cell.marked { 'X' } else { ' ' });
            }
            println!();
        }

        Ok(println!(
            "Round: {} - Final number: {}",
            self.state.round, self.state.final_number
        ))
    }
}

impl Board<FinalState> {
    fn is_bingo(&self) -> bool {
        self.state.round > 0
    }
    fn get_bingo_round(&self) -> usize {
        self.state.round
    }
    fn get_points(&self) -> usize {
        // iterate board and sum not marked cells, then multiply by the round
        if self.state.round == 0 {
            return 0;
        }
        let unmarked_cells = self.grid.iter().flatten().filter(|c| !c.marked);
        unmarked_cells.fold(0, |acc, cell| acc + cell.number) * self.state.final_number
    }
}

impl Board<InitialState> {
    fn parse(input: &mut Input) -> Self {
        let mut grid = vec![Vec::new(); 5];
        for index in 0..5 {
            let line = input
                .lines
                .next()
                .expect("Failed to read line")
                .expect("Failed to parse line");

            let row = line
                .split_whitespace()
                .map(|n| n.parse::<usize>().expect("Failed to parse number"))
                .collect::<Vec<usize>>();
            grid[index] = row.iter().map(|n| Cell::new(*n)).collect();
        }

        Board {
            grid,
            state: InitialState {},
        }
    }

    fn play(&mut self, numbers: &Vec<usize>) -> Board<FinalState> {
        for (index, number) in numbers.iter().enumerate() {
            for row in self.grid.iter_mut() {
                for cell in row.iter_mut() {
                    cell.update(*number);
                }
            }

            // asume is a square matrix
            for x in 0..self.grid.len() {
                // if all cells in row or column are marked, then we have a bingo
                let row_bingo = self.grid[x].iter().all(|c| c.marked);
                let column_bingo = self.grid.iter().all(|row| row[x].marked);
                if row_bingo || column_bingo {
                    return Board {
                        grid: self.grid.clone(),
                        state: FinalState {
                            round: index + 1,
                            final_number: *number,
                        },
                    };
                }
            }
        }

        Board {
            grid: self.grid.clone(),
            state: FinalState {
                round: 0,
                final_number: 0,
            },
        }
    }
}

fn main() {
    let winner_board = find_winner_board();
    println!(
        "winner board {} with points {}",
        winner_board,
        winner_board.get_points()
    );

    let last_winner_board = find_last_winner_board();
    println!(
        "last winner board {} with points {}",
        last_winner_board,
        last_winner_board.get_points()
    );
}
