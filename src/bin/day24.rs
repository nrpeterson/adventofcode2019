use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use adventofcode2019::build_main;
use adventofcode2019::grid::{Grid, Position};

fn next_grid(grid: Grid<bool>) -> Grid<bool> {
    let mut result = Grid::new(vec![vec![false; 5]; 5]);

    for i in 0..5 {
        for j in 0..5 {
            let p = Position(i, j);
            let num_adj = grid.neighbors(p).into_iter()
                .filter(|n| grid[*n])
                .count();

            if grid[p] && num_adj != 1 {
                result[p] = false;
            }
            else if !grid[p] && (num_adj == 1 || num_adj == 2) {
                result[p] = true;
            }
            else {
                result[p] = grid[p];
            }
        }
    }

    result
}

fn signature(grid: &Grid<bool>) -> usize {
    let mut result = 0;

    for row in grid.vals.iter().rev() {
        for &elem in row.iter().rev() {
            result *= 2;
            if elem { result += 1; }
        }
    }

    result
}

fn parse_simple(input: &str) -> Grid<bool> {
    let vals = input.lines()
        .map(|line| line.chars().map(|c| c == '#').collect_vec())
        .collect_vec();

    Grid::new(vals)
}

fn part1(input: &str) -> usize {
    let mut grid = parse_simple(input);
    let mut seen = HashSet::new();

    loop {
        let s = signature(&grid);
        if !seen.insert(s) {
            return s;
        }

        grid = next_grid(grid);
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Location { i: usize, j: usize, level: isize }

impl Location {
    fn new(i: usize, j: usize, level: isize) -> Location {
        Location { i, j, level }
    }

    fn neighbors(&self) -> Vec<Location> {
        let mut neighbors = Vec::new();
        let Location { i, j, level } = *self;

        if i > 0 && (i, j) != (3, 2) {
            neighbors.push(Location::new(i-1, j, level));
        }

        if i < 4 && (i, j) != (1, 2) {
            neighbors.push(Location::new(i+1, j, level));
        }

        if j > 0 && (i, j) != (2, 3) {
            neighbors.push(Location::new(i, j-1, level));
        }

        if j < 4 && (i, j) != (2, 1) {
            neighbors.push(Location::new(i, j+1, level));
        }

        if i == 0 {
            neighbors.push(Location::new(1, 2, level - 1));
        }

        if i == 4 {
            neighbors.push(Location::new(3, 2, level - 1));
        }

        if j == 0 {
            neighbors.push(Location::new(2, 1, level - 1));
        }

        if j == 4 {
            neighbors.push(Location::new(2, 3, level - 1));
        }

        if (i, j) == (1, 2) {
            neighbors.extend((0..5).map(|j0| Location::new(0, j0, level + 1)));
        }

        if (i, j) == (3, 2) {
            neighbors.extend((0..5).map(|j0| Location::new(4, j0, level + 1)));
        }

        if (i, j) == (2, 1) {
            neighbors.extend((0..5).map(|i0| Location::new(i0, 0, level + 1)));
        }

        if (i, j) == (2, 3) {
            neighbors.extend((0..5).map(|i0| Location::new(i0, 4, level + 1)));
        }

        neighbors
    }
}

fn next_spaces(cur: HashSet<Location>) -> HashSet<Location> {
    let mut counts = HashMap::new();

    for loc in cur.iter() {
        for neighbor in loc.neighbors() {
            *counts.entry(neighbor).or_insert(0) += 1;
        }
    }

    counts.into_iter()
        .filter(|(loc, n)| {
            let is_bug = cur.contains(loc);
            (is_bug && *n == 1) || (!is_bug && (*n == 1 || *n == 2))
        })
        .map(|(loc, _)| loc)
        .collect()
}

fn part2(input: &str) -> usize {
    let mut bugs = input.lines().enumerate()
        .flat_map(|(i, line)| {
            line.chars().enumerate()
                .map(move |(j, c)| (Location::new(i, j, 0), c == '#'))
        })
        .filter(|(_, b)| *b)
        .map(|(loc, _)| loc)
        .collect::<HashSet<_>>();

    for _ in 0..200 {
        bugs = next_spaces(bugs);
    }

    bugs.len()
}

build_main!("day24.txt", "Part 1" => part1, "Part 2" => part2);