use adventofcode2019::build_main;
use adventofcode2019::grid::{Grid, Position};
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt::{Debug, Formatter};

enum Space {
    Open,
    Wall,
    Door(usize)
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct KeySet(usize);

impl Debug for KeySet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "KeySet{:?}", self.to_vec())
    }
}

impl KeySet {
    fn contains(&self, key: usize) -> bool {
        self.0 & (1 << key) != 0
    }

    fn contains_all(&self, that: KeySet) -> bool {
        self.0 & that.0 == that.0
    }

    fn insert(&mut self, key: usize) {
        self.0 |= 1 << key;
    }

    fn to_vec(&self) -> Vec<usize> {
        (0..26).filter(|&k| self.contains(k)).collect_vec()
    }
}

const ALL_KEYS: KeySet = KeySet(0b11111111111111111111111111);


#[derive(Clone, Hash, Eq, PartialEq, Debug)]
struct Node {
    positions: Vec<Position>,
    keys: KeySet
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct HeapElem {
    node: Node,
    steps: usize
}

impl Ord for HeapElem {
    fn cmp(&self, other: &Self) -> Ordering {
        other.steps.cmp(&self.steps)
    }
}

impl PartialOrd for HeapElem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Scene {
    grid: Grid<Space>,
    keys: HashMap<Position, usize>,
    key_positions: Vec<Position>
}

impl Scene {
    fn parse(input: &str) -> (Scene, Vec<Position>) {
        let spaces = input.lines()
            .map(|line| {
                line.chars()
                    .map(|c| {
                        match c {
                            '.' | '@' => Space::Open,
                            '#' => Space::Wall,
                            c if c.is_ascii_lowercase() => Space::Open,
                            c => Space::Door(c as usize - 'A' as usize)
                        }
                    })
                    .collect_vec()
            }).collect_vec();

        let grid = Grid::new(spaces);

        let positions = input.lines().enumerate()
            .flat_map(|(i, line)| {
                line.chars().enumerate()
                    .map(move |(j, c)| (Position(i, j), c))
            })
            .filter(|(_, c)| *c == '@')
            .map(|(p, _)| p)
            .collect_vec();

        let mut keys = HashMap::new();
        let mut key_positions = vec![Position(0, 0); 26];

        for (i, line) in input.lines().enumerate() {
            for (j, c) in line.chars().enumerate() {
                if c.is_ascii_lowercase() {
                    let key = (c as usize) - ('a' as usize);
                    keys.insert(Position(i, j), key);
                    key_positions[key] = Position(i, j);
                }
            }
        }

        let scene = Scene { grid, keys, key_positions };
        (scene, positions)
    }

    fn min_paths(&self, from: Position) -> Vec<Vec<(KeySet, usize)>> {
        let mut cache = vec![HashMap::new(); self.keys.len()];

        let mut stack = Vec::new();
        stack.push((from, HashSet::from([from]), 0, KeySet(0)));

        while let Some((position, visited, steps, keys)) = stack.pop() {
            if let Some(&key) = self.keys.get(&position) {
                let cur = cache[key].entry(keys).or_insert(usize::MAX);

                if steps < *cur {
                    *cur = steps;
                }
            }

            for nbr in self.grid.neighbors(position) {
                if visited.contains(&nbr) {
                    continue;
                }

                match self.grid[nbr] {
                    Space::Open => {
                        let mut new_visited = visited.clone();
                        new_visited.insert(nbr);
                        stack.push((nbr, new_visited, steps + 1, keys));
                    },
                    Space::Door(key) => {
                        let mut new_visited = visited.clone();
                        new_visited.insert(nbr);
                        let mut new_keys = keys;
                        new_keys.insert(key);
                        stack.push((nbr, new_visited, steps + 1, new_keys));
                    },
                    _ => ()
                }
            }
        }

        let mut result = Vec::new();
        for map in cache {
            let mut for_key = Vec::new();
            for (keys, steps) in map {
                for_key.push((keys, steps));
            }
            result.push(for_key);
        }

        result
    }

    fn min_steps(&self, from: Node) -> usize {
        let mut dists = HashMap::new();
        dists.insert(from.clone(), 0);

        let mut q = BinaryHeap::new();
        q.push(HeapElem { node: from, steps: 0 });

        let mut min_paths = HashMap::new();

        while let Some(HeapElem { node, steps }) = q.pop() {
            if node.keys == ALL_KEYS {
                return steps;
            }

            let cur_dist = dists.entry(node.clone()).or_insert(usize::MAX);
            if steps > *cur_dist {
                continue;
            }

            for (n, &pos) in node.positions.iter().enumerate() {
                let paths = min_paths.entry(pos).or_insert_with(|| self.min_paths(pos));
                let mut choices = Vec::new();
                for key in 0..26 {
                    if node.keys.contains(key) {
                        continue;
                    }

                    let min_dist = paths[key].iter()
                        .filter(|&&(needed_keys, _)| node.keys.contains_all(needed_keys))
                        .map(|(_, v)| *v)
                        .min();

                    if let Some(d) = min_dist {
                        choices.push((self.key_positions[key], key, d));
                    }
                }

                for (next_pos, key, d) in choices {
                    let mut next_node = node.clone();
                    next_node.keys.insert(key);
                    next_node.positions[n] = next_pos;
                    let next_cur_dist = dists.entry(next_node.clone()).or_insert(usize::MAX);
                    if steps + d < *next_cur_dist {
                        *next_cur_dist = steps + d;

                        let next_state = HeapElem { node: next_node, steps: steps + d };
                        q.push(next_state);
                    }
                }
            }
        }

        unreachable!()
    }

}

fn part1(input: &str) -> usize {
    let (scene, start) = Scene::parse(input);
    let from = Node { positions: start, keys: KeySet(0) };

    scene.min_steps(from)
}

fn part2(input: &str) -> usize {
    let mut buffer = input.lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let (i, j) = (0..buffer.len()).cartesian_product(0..buffer[0].len())
        .find(|&(i, j)| buffer[i][j] == '@')
        .unwrap();

    for (i0, j0) in [(i-1, j-1), (i-1, j+1), (i+1, j-1), (i+1, j+1)] {
        buffer[i0][j0] = '@';
    }

    for (i0, j0) in [(i-1, j), (i, j-1), (i, j), (i, j+1), (i+1, j)] {
        buffer[i0][j0] = '#';
    }

    let s = buffer.into_iter()
        .map(|line| line.into_iter().collect::<String>())
        .join("\n");

    let (scene, start) = Scene::parse(&s);
    let from = Node { positions: start, keys: KeySet(0) };
    scene.min_steps(from)
}

build_main!("day18.txt", "Part 1" => part1, "Part 2" => part2);