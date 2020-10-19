#[macro_use]
extern crate lazy_static;

use std::convert::TryInto;
use std::fmt;

pub mod render;
pub mod solve;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SudokuCell {
    Solved(u8),
    Unsolved([bool; 9]),
}

impl SudokuCell {
    pub fn new(val: u8) -> SudokuCell {
        if (val >= 1) && (val <= 9) {
            Self::Solved(val)
        } else {
            Self::Unsolved([true; 9])
        }
    }

    pub fn candidates(&self) -> Option<Vec<u8>> {
        match self {
            Self::Solved(_) => None,
            Self::Unsolved(possible) => {
                let mut candx: Vec<u8> = vec![];
                for i in 0..9 {
                    if possible[i] {
                        candx.push((i + 1).try_into().unwrap());
                    }
                }
                Some(candx)
            }
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct SudokuGrid {
    grid: [SudokuCell; 81],
}

impl SudokuGrid {
    pub fn new() -> SudokuGrid {
        SudokuGrid {
            grid: [SudokuCell::new(0); 81],
        }
    }

    fn pos(x: u8, y: u8) -> usize {
        if (x == 0) || (x > 9) || (y == 0) || (y > 9) {
            panic!("out of bounds");
        }
        ((x - 1) + (y - 1) * 9).into()
    }

    pub fn box_range(coord: u8) -> std::ops::Range<u8> {
        let low = 1 + ((coord - 1) / 3) * 3;
        low..low + 3
    }

    pub fn get_cell(&self, x: u8, y: u8) -> &SudokuCell {
        &self.grid[SudokuGrid::pos(x, y)]
    }

    pub fn set_cell(&mut self, x: u8, y: u8, cell: SudokuCell) {
        self.grid[SudokuGrid::pos(x, y)] = cell;
    }

    pub fn from_string(cell_values: &str) -> Result<SudokuGrid> {
        if cell_values.len() != 81 {
            Err(Error::new("Puzzle string must have 81 digits"))
        } else {
            let char_vec: Vec<char> = cell_values.chars().collect();
            let char_arr: [char; 81] = (char_vec[0..81]).try_into().unwrap();
            Ok(SudokuGrid::from_chars(&char_arr))
        }
    }

    pub fn from_chars(cell_values: &[char; 81]) -> SudokuGrid {
        let mut cells: Vec<SudokuCell> = Vec::new();
        for c in cell_values.iter() {
            let n: u8 = match c.to_digit(10) {
                Some(n) => {
                    if (n >= 1) && (n <= 9) {
                        n.try_into().unwrap()
                    } else {
                        0
                    }
                }
                None => 0,
            };
            cells.push(SudokuCell::new(n));
        }
        SudokuGrid {
            grid: cells[0..81].try_into().unwrap(),
        }
    }
}

#[derive(Debug)]
pub struct Error {
    details: String,
}

impl Error {
    fn new(msg: &str) -> Error {
        Error {
            details: msg.to_string(),
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        &self.details
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
