use adventofcode2019::build_main_res;
use adventofcode2019::intcode::cpu::CPU;
use adventofcode2019::intcode::io::{IProvider, OProvider};
use adventofcode2019::intcode::IntcodeError::LogicError;
use adventofcode2019::intcode::IntcodeState::Continue;
use adventofcode2019::intcode::{IntcodeResult, IntcodeState, Runnable};
use adventofcode2019::points::Point2D;
use itertools::{chain, Itertools};
use std::cmp::{max, min};
use std::collections::HashSet;

struct Robot {
    position: Point2D,
    direction: Point2D,
    is_white: HashSet<Point2D>,
    visited: HashSet<Point2D>,
    awaiting_color: bool
}

impl Robot {
    fn new() -> Robot {
        let position = Point2D(0, 0);
        let direction = Point2D(0, 1);
        let is_white = HashSet::new();
        let visited = HashSet::from([position]);
        let awaiting_color = true;
        Robot { position, direction, is_white, visited, awaiting_color }
    }
}

impl IProvider for Robot {
    type PInput = isize;
    type RInput = isize;

    fn provide_input<O>(&mut self) -> IntcodeResult<(IntcodeState<O>, Option<isize>)> {
        match self.is_white.contains(&self.position) {
            true => Ok((Continue, Some(1))),
            false => Ok((Continue, Some(0)))
        }
    }

    fn receive_input(&mut self, input: Self::RInput) -> IntcodeResult<()> {
        Err(LogicError(format!("Didn't expect to receive any input; received {input}")))
    }
}

impl OProvider for Robot {
    type POutput = isize;
    type ROutput = isize;

    fn handle_output(&mut self, output: Self::ROutput) -> IntcodeResult<IntcodeState<isize>> {
        if self.awaiting_color {
            if output == 1 {
                self.is_white.insert(self.position);
            }
            else {
                self.is_white.remove(&self.position);
            }
            self.awaiting_color = false;
            Ok(Continue)
        }
        else {
            let Point2D(x, y) = self.direction;
            self.direction = if output == 0 { Point2D(-y, x) } else { Point2D(y, -x) };
            self.position = self.position + self.direction;
            self.visited.insert(self.position);
            self.awaiting_color = true;
            Ok(Continue)
        }
    }
}

fn part1(input: &str) -> IntcodeResult<usize> {
    let robot = Robot::new();
    let cpu = CPU::parse(input)?;
    let mut system = cpu.wrap(robot);
    system.run()?;
    Ok(system.outer.visited.len())
}

fn part2(input: &str) -> IntcodeResult<String> {
    let mut robot = Robot::new();
    robot.is_white.insert(Point2D(0, 0));
    let cpu = CPU::parse(input)?;
    let mut system = cpu.wrap(robot);
    system.run()?;

    let (x_min, x_max, y_min, y_max) = system.outer.is_white.iter()
        .fold((isize::MAX, isize::MIN, isize::MAX, isize::MIN),
              |(x0, x1, y0, y1), &Point2D(x, y)| {
                  (min(x0, x), max(x1, x), min(y0, y), max(y1, y))
              }
        );

    let mut message = vec![
        vec!['.'; (x_max - x_min + 1) as usize];
        (y_max - y_min + 1) as usize
    ];

    for &Point2D(x, y) in system.outer.is_white.iter() {
        let i = (y_max - y) as usize;
        let j = (x - x_min) as usize;
        message[i][j] = '#';
    }

    let msg = chain!(std::iter::once(vec![]), message.into_iter())
        .map(|row| row.into_iter().collect::<String>())
        .join("\n");

    Ok(msg)
}

build_main_res!("day11.txt", "Part 1" => part1, "Part 2" => part2);