use std::collections::{HashMap, HashSet, VecDeque};
use itertools::{chain, repeat_n, Itertools};
use nom::AsChar;
use adventofcode2019::build_main;
use adventofcode2019::grid::{Grid, Position};

struct Graph {
    edges: HashMap<Position, Vec<Position>>,
    start: Position,
    end: Position
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Node {
    position: Position,
    depth: usize
}

impl Node {
    fn new(position: Position, depth: usize) -> Node {
        Node { position, depth }
    }
}

struct Scene {
    grid: Grid<char>,
    start: Position,
    end: Position,
    inner_portals_by_name: HashMap<(char, char), Position>,
    outer_portals_by_name: HashMap<(char, char), Position>,
    inner_portals_by_pos: HashMap<Position, (char, char)>,
    outer_portals_by_pos: HashMap<Position, (char, char)>
}

impl Scene {
    fn edges(&self, node: Node, recurse: bool) -> Vec<Node> {
        if self.grid[node.position] != '.' {
            return vec![];
        }

        let mut results = Vec::new();

        if let Some(name) = self.inner_portals_by_pos.get(&node.position) {
            let target_pos = self.outer_portals_by_name[name];
            let target_depth = if recurse { node.depth + 1 } else { node.depth };
            results.push(Node::new(target_pos, target_depth));
        }

        if let Some(name) = self.outer_portals_by_pos.get(&node.position) {
            if !recurse || node.depth > 0 {
                let target_pos = self.inner_portals_by_name[name];
                let target_depth = if recurse { node.depth - 1 } else { node.depth };
                results.push(Node::new(target_pos, target_depth));
            }
        }

        for nbr in self.grid.neighbors(node.position) {
            if self.grid[nbr] == '.' {
                results.push(Node::new(nbr, node.depth));
            }
        }

        results
    }
}

fn parse_input(input: &str) -> Scene {
    let max_cols = input.lines().map(|line| line.len()).max().unwrap();

    let buffer = input.lines()
        .map(|line| {
            let mut row = line.chars().collect_vec();
            row.extend(repeat_n(' ', max_cols - row.len()));
            row
        })
        .collect_vec();

    let grid = Grid::new(buffer);

    let row_count = grid.iter_row(2).filter(|&&c| c != ' ').count();
    let mut i0 = 2;
    while grid.iter_row(i0).filter(|&&c| c != ' ').count() == row_count {
        i0 += 1;
    }

    let mut i1 = i0 + 1;
    while grid.iter_row(i1 + 1).filter(|&&c| c != ' ').count() != row_count {
        i1 += 1;
    }

    let col_count = grid.iter_col(2).filter(|&&c| c != ' ').count();

    let mut j0 = 2;
    while grid.iter_col(j0).filter(|&&c| c != ' ').count() == col_count {
        j0 += 1;
    }

    let mut j1 = j0 + 1;
    while grid.iter_col(j1 + 1).filter(|&&c| c != ' ').count() != col_count {
        j1 += 1;
    }

    let mut edges: HashMap<Position, Vec<Position>> = HashMap::new();

    (2..grid.rows-2).cartesian_product(2..grid.cols-2)
        .map(|(i, j)| Position(i, j))
        .filter(|&pos| grid[pos] == '.')
        .for_each(|pos| {
            for nbr in grid.neighbors(pos) {
                if grid[nbr] == '.' {
                    edges.entry(pos).or_default().push(nbr);
                }
            }
        });

    let m = grid.rows;
    let n = grid.cols;
    let mut start = Position(0, 0);
    let mut end = Position(0, 0);

    let mut outer_portals_by_name = HashMap::new();
    let mut outer_portals_by_pos = HashMap::new();
    let mut inner_portals_by_name = HashMap::new();
    let mut inner_portals_by_pos = HashMap::new();

    chain!(
        (0..n).map(|j| (Position(0, j), Position(1, j), Position(2, j))),
        (0..m).map(|i| (Position(i, 0), Position(i, 1), Position(i, 2))),
        (0..n).map(|j| (Position(m-2, j), Position(m-1, j), Position(m-3, j))),
        (0..m).map(|i| (Position(i, n-2), Position(i, n-1), Position(i, n-3)))
    )
        .map(|(p0, p1, p2)| (grid[p0], grid[p1], p2))
        .filter(|&(a, b, _)| a.is_alphabetic() && b.is_alphabetic())
        .for_each(|(a, b, pos)| {
            if (a, b) == ('A', 'A') {
                start = pos;
            }
            else if (a, b) == ('Z', 'Z') {
                end = pos;
            }
            else {
                outer_portals_by_name.insert((a, b), pos);
                outer_portals_by_pos.insert(pos, (a, b));
            }
        });

    chain!(

        (j0..=j1).map(|j| (Position(i0, j), Position(i0+1, j), Position(i0-1, j))),
        (j0..=j1).map(|j| (Position(i1-1, j), Position(i1, j), Position(i1+1, j))),
        (i0..=i1).map(|i| (Position(i, j0), Position(i, j0+1), Position(i, j0-1))),
        (i0..=i1).map(|i| (Position(i, j1-1), Position(i, j1), Position(i, j1+1))),
    )
        .map(|(p0, p1, p2)| (grid[p0], grid[p1], p2))
        .filter(|&(a, b, _)| a.is_alphabetic() && b.is_alphabetic())
        .for_each(|(a, b, pos)| {
            if (a, b) == ('A', 'A') {
                start = pos;
            }
            else if (a, b) == ('Z', 'Z') {
                end = pos;
            }
            else {
                inner_portals_by_name.insert((a, b), pos);
                inner_portals_by_pos.insert(pos, (a, b));
            }
        });

    Scene {
        grid,
        start,
        end,
        inner_portals_by_name,
        outer_portals_by_name,
        inner_portals_by_pos,
        outer_portals_by_pos
    }
}

fn steps(scene: &Scene, recurse: bool) -> usize {
    let start = Node::new(scene.start, 0);
    let end = Node::new(scene.end, 0);

    let mut queue = VecDeque::new();
    queue.push_back((start, 0));

    let mut seen = HashSet::new();
    seen.insert(start);

    while let Some((v, d)) = queue.pop_front() {
        if v == end {
            return d;
        }

        for n in scene.edges(v, recurse).into_iter() {
            if seen.insert(n) {
                queue.push_back((n, d + 1));
            }
        }
    }

    unreachable!()
}

fn part1(input: &str) -> usize {
    let scene = parse_input(input);
    steps(&scene, false)
}

fn part2(input: &str) -> usize {
    let scene = parse_input(input);
    steps(&scene, true)
}

build_main!("day20.txt", "Part 1" => part1, "Part 2" => part2);