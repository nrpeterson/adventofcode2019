use std::ops::{Index, IndexMut};
use itertools::Itertools;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Direction { Up, Down, Left, Right }

impl Direction {
    pub fn clockwise(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up
        }
    }

    pub fn counterclockwise(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Position(pub usize, pub usize);

impl Position {
    pub fn step(&self, direction: Direction) -> Option<Position> {
        match (direction, self.0, self.1) {
            (Direction::Up, 0, _) => None,
            (Direction::Up, i, j) => Some(Position(i - 1, j)),
            (Direction::Down, i, j) => Some(Position(i + 1, j)),
            (Direction::Left, _, 0) => None,
            (Direction::Left, i, j) => Some(Position(i, j - 1)),
            (Direction::Right, i, j) => Some(Position(i, j + 1))
        }
    }
}

pub struct Grid<T> {
    pub rows: usize,
    pub cols: usize,
    pub vals: Vec<Vec<T>>
}

impl<T> Grid<T> {
    pub fn new(vals: Vec<Vec<T>>) -> Grid<T> {
        let rows = vals.len();
        let cols = vals[0].len();

        vals.iter().for_each(|row| assert_eq!(row.len(), cols));
        Grid { rows, cols, vals }
    }

    pub fn iter_row(&self, row: usize) -> GridRowIter<T> {
        GridRowIter { grid: self, row, cur_col: 0 }
    }

    pub fn iter_col(&self, col: usize) -> GridColIter<T> {
        GridColIter { grid: self, col, cur_row: 0 }
    }

    pub fn step_from(&self, Position(i, j): Position, direction: Direction) -> Option<Position> {
        match (direction, i, j) {
            (Direction::Up, i, j) if i > 0 => Some(Position(i - 1, j)),
            (Direction::Down, i, j) if i + 1 < self.rows => Some(Position(i + 1, j)),
            (Direction::Left, i, j) if j > 0 => Some(Position(i, j - 1)),
            (Direction::Right, i, j) if j + 1 < self.cols => Some(Position(i, j + 1)),
            _ => None
        }
    }

    pub fn neighbors(&self, position: Position) -> Vec<Position> {
        [Direction::Up, Direction::Down, Direction::Left, Direction::Right].into_iter()
            .filter_map(|dir| self.step_from(position, dir))
            .collect_vec()
    }
}

impl Grid<char> {
    pub fn from_string(s: &str) -> Grid<char> {
        let v = s.lines().map(|line| line.chars().collect_vec()).collect_vec();
        Grid::new(v)
    }
}

impl<T> Index<Position> for Grid<T> {
    type Output = T;
    fn index(&self, Position(i, j): Position) -> &Self::Output {
        &self.vals[i][j]
    }
}

impl<T> IndexMut<Position> for Grid<T> {
    fn index_mut(&mut self, Position(i, j): Position) -> &mut Self::Output {
        &mut self.vals[i][j]
    }
}

pub struct GridRowIter<'a, T> {
    grid: &'a Grid<T>,
    row: usize,
    cur_col: usize
}

impl<'a, T> Iterator for GridRowIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.cur_col == self.grid.cols { None }
        else {
            let result = &self.grid.vals[self.row][self.cur_col];
            self.cur_col += 1;
            Some(result)
        }
    }
}

pub struct GridColIter<'a, T> {
    grid: &'a Grid<T>,
    col: usize,
    cur_row: usize
}

impl<'a, T> Iterator for GridColIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.cur_row == self.grid.rows { None }
        else {
            let result = &self.grid.vals[self.cur_row][self.col];
            self.cur_row += 1;
            Some(result)
        }
    }
}