use std::collections::{HashMap, VecDeque};
use adventofcode2019::build_main;
use adventofcode2019::intcode::Computer;
use adventofcode2019::points::{Direction2D, Point2D};
use adventofcode2019::points::Direction2D::{Down, Left, Right, Up};
use crate::Response::*;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Response {
    Wall,
    Open,
    OxygenSystem
}

fn dir_code(direction: Direction2D) -> isize {
    match direction {
        Up => 1,
        Down => 2,
        Left => 3,
        Right => 4
    }
}

#[derive(Copy, Clone)]
struct BFSResult {
    distance: usize,
    step_dir: Option<Direction2D>
}

struct Scene {
    computer: Computer,
    robot: Point2D,
    explored: HashMap<Point2D, Response>,
    unexplored_via: Vec<(Point2D, Point2D, Direction2D)>,
    oxygen_system: Option<Point2D>
}

impl Scene {
    fn parse(input: &str) -> Scene {
        let computer = Computer::parse(input, vec![]);
        let robot = Point2D(0, 0);
        let explored = HashMap::from([(Point2D(0, 0), Response::Open)]);
        let unexplored_via = Vec::from([
            (Point2D(-1, 0), Point2D(0, 0), Left),
            (Point2D(1, 0), Point2D(0, 0), Right),
            (Point2D(0, -1), Point2D(0, 0), Down),
            (Point2D(0, 1), Point2D(0, 0), Up)
        ]);
        let oxygen_system = None;

        Scene { computer, robot, explored, unexplored_via, oxygen_system }
    }

    fn bfs_from(&self, from: Point2D, target: Option<Point2D>) -> HashMap<Point2D, BFSResult> {
        let mut results = HashMap::new();
        results.insert(from, BFSResult { distance: 0, step_dir: None });

        let mut queue = VecDeque::new();
        queue.push_back((from, BFSResult { distance: 0, step_dir: None }));

        while let Some((point, result)) = queue.pop_front() {
            for direction in [Up, Down, Left, Right] {
                let neighbor = point + direction.to_step();
                if results.contains_key(&neighbor) {
                    continue;
                }
                let kind = self.explored.get(&neighbor);
                if kind.is_none() || kind.is_some_and(|&k| k == Wall) {
                    continue;
                }

                let neighbor_result = BFSResult {
                    distance: result.distance + 1,
                    step_dir: Some(direction)
                };
                queue.push_back((neighbor, neighbor_result));
                results.insert(neighbor, neighbor_result);

                if let Some(t) = target {
                    if neighbor == t {
                        break
                    }
                }
            }
        }

        results
    }

    fn path_to(&mut self, target: Point2D) -> Option<Vec<Direction2D>> {
        if target == self.robot { return Some(Vec::new()); }

        let bfs = self.bfs_from(self.robot, Some(target));

        let mut point = target;
        let mut result = Vec::new();

        while let Some(&BFSResult { step_dir: Some(dir), .. }) = bfs.get(&point) {
            result.push(dir);
            point -= dir.to_step();
        }

        if result.is_empty() {
            None
        }
        else {
            result.reverse();
            Some(result)
        }
    }

    fn step(&mut self) -> Option<(Point2D, Response)> {
        let (target, via, target_dir) = self.unexplored_via.pop()?;

        for dir in self.path_to(via)? {
            self.computer.input(dir_code(dir));
            self.computer.next_output().expect("a direction");
            self.robot += dir.to_step();
        }

        assert_eq!(self.robot, via);

        self.computer.input(dir_code(target_dir));
        match self.computer.next_output()? {
            0 => {
                self.explored.insert(target, Response::Wall);
                Some((target, Response::Wall))
            },
            1 => {
                self.explored.insert(target, Open);
                self.robot = target;
                for d in [Up, Down, Left, Right] {
                    let neighbor = d.to_step() + self.robot;
                    if !self.explored.contains_key(&neighbor) {
                        self.unexplored_via.push((neighbor, self.robot, d));
                    }
                }
                Some((target, Open))
            },
            2 => {
                self.oxygen_system = Some(target);
                self.explored.insert(target, OxygenSystem);
                self.robot = target;
                Some((target, OxygenSystem))
            },
            _ => panic!()
        }
    }

    fn explore(&mut self) {
        while let Some(_) = self.step() { }
        assert!(self.unexplored_via.is_empty());
    }

    fn distance(&self, from: Point2D, to: Point2D) -> usize {
        let results = self.bfs_from(from, Some(to));
        results[&to].distance
    }
}

fn part1(input: &str) -> usize {
    let mut scene = Scene::parse(input);
    scene.explore();
    let o2 = scene.oxygen_system.expect("oxygen system");
    scene.distance(Point2D(0, 0), o2)
}

fn part2(input: &str) -> usize {
    let mut scene = Scene::parse(input);
    scene.explore();
    let o2 = scene.oxygen_system.expect("oxygen system");

    let bfs = scene.bfs_from(o2, None);
    bfs.into_values()
        .map(|r| r.distance)
        .max()
        .unwrap()
}

build_main!("day15.txt", "Part 1" => part1, "Part 2" => part2);