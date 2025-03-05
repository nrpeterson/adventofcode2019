use crate::Response::*;
use adventofcode2019::computer::io::{InputProvider, OutputHandler};
use adventofcode2019::computer::IntcodeError::LogicError;
use adventofcode2019::computer::{IntcodeResult, Runnable, System};
use adventofcode2019::points::Direction2D::{Down, Left, Right, Up};
use adventofcode2019::points::{Direction2D, Point2D};
use adventofcode2019::build_main_res;
use std::collections::{HashMap, VecDeque};

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

struct Robot {
    position: Point2D,
    explored: HashMap<Point2D, Response>,
    oxygen_system: Option<Point2D>,
    cur_path: VecDeque<Direction2D>,
    cur_target: Option<Point2D>
}

impl Robot {
    fn new() -> Robot {
        let position = Point2D(0, 0);
        let explored = HashMap::from([(Point2D(0, 0), Response::Open)]);
        let oxygen_system = None;
        let cur_path = VecDeque::new();
        let cur_target = None;

        Robot { position, explored, oxygen_system, cur_path, cur_target }
    }

    fn bfs_until_unseen(&self, from: Point2D) -> (HashMap<Point2D, BFSResult>, Option<Point2D>) {
        let mut results = HashMap::new();
        results.insert(from, BFSResult { distance: 0, step_dir: None });

        let mut queue = VecDeque::new();
        queue.push_back((from, BFSResult { distance: 0, step_dir: None }));

        let mut target = None;

        while let Some((point, result)) = queue.pop_front() {
            for dir in [Up, Down, Left, Right] {
                let neighbor = point + dir.to_step();

                if results.contains_key(&neighbor) || self.explored.get(&neighbor) == Some(&Wall) {
                    continue;
                }

                let r = BFSResult { distance: result.distance + 1, step_dir: Some(dir) };
                results.insert(neighbor, r);
                queue.push_back((neighbor, r));

                if !self.explored.contains_key(&neighbor) {
                    target = Some(neighbor);
                    break;
                }
            }

            if !target.is_none() {
                break;
            }
        }

        (results, target)
    }

    fn distance(&mut self, from: Point2D, to: Point2D) -> Option<usize> {
        let (bfs, _) = self.bfs_until_unseen(from);
        let r = bfs.get(&to)?;
        Some(r.distance)
    }
}

impl InputProvider for Robot {
    fn get(&mut self) -> IntcodeResult<Option<isize>> {
        if self.cur_path.is_empty() {
            let (bfs, target) = self.bfs_until_unseen(self.position);

            if let Some(t) = target {
                let mut path = Vec::new();
                let mut point = t;

                while let Some(&BFSResult { step_dir: Some(dir), ..}) = bfs.get(&point) {
                    path.push(dir);
                    point -= dir.to_step();
                }

                path.reverse();
                self.cur_path.extend(path);
            }
        }

        if let Some(dir) = self.cur_path.pop_front() {
            self.cur_target = Some(self.position + dir.to_step());
            Ok(Some(dir_code(dir)))
        }
        else {
            Ok(None)
        }
    }
}

impl OutputHandler for Robot {
    fn push(&mut self, v: isize) -> IntcodeResult<()> {

        let msg = "Shouldn't get output without a target...".to_string();
        let target = self.cur_target.ok_or(LogicError(msg))?;

        let response = match v {
            0 => Wall,
            1 => Open,
            2 => OxygenSystem,
            _ => return Err(LogicError("bad output from program".to_string()))
        };

        self.explored.insert(target, response);

        if response != Wall {
            self.position = target;
        }

        if response == OxygenSystem {
            self.oxygen_system = Some(target);
        }

        Ok(())
    }
}

fn part1(input: &str) -> IntcodeResult<usize> {
    let mut system = System::parse(input, Robot::new())?;
    system.run()?;

    let o2 = system.io.oxygen_system.ok_or(LogicError("Didn't find O2 system".to_string()))?;
    system.io
        .distance(Point2D(0, 0), o2)
        .ok_or(LogicError("Didn't find a path from (0, 0) to O2".to_string()))
}

fn part2(input: &str) -> IntcodeResult<usize> {
    let mut system = System::parse(input, Robot::new())?;
    system.run()?;

    let o2 = system.io.oxygen_system.ok_or(LogicError("Didn't find O2 system".to_string()))?;

    let (bfs, target) = system.io.bfs_until_unseen(o2);
    assert!(target.is_none());

    let result = bfs.into_values()
        .map(|r| r.distance)
        .max()
        .unwrap();

    Ok(result)
}

build_main_res!("day15.txt", "Part 1" => part1, "Part 2" => part2);