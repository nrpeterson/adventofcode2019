use crate::GameObject::{Ball, Block, Empty, HorizontalPaddle, Wall};
use adventofcode2019::build_main_res;
use adventofcode2019::intcode::io::{InputProvider, OutputHandler};
use adventofcode2019::intcode::{IntcodeResult, Runnable, System};
use adventofcode2019::points::Point2D;
use std::collections::HashMap;

#[derive(Copy, Clone, Eq, PartialEq)]
enum GameObject {
    Empty,
    Wall,
    Block,
    HorizontalPaddle,
    Ball
}

impl GameObject {
    fn from_id(i: isize) -> GameObject {
        match i {
            0 => Empty,
            1 => Wall,
            2 => Block,
            3 => HorizontalPaddle,
            4 => Ball,
            _ => panic!("Invalid object code {i}")
        }
    }
}

struct ArcadeCabinet {
    screen: HashMap<Point2D, GameObject>,
    score: isize,
    ball: Point2D,
    paddle: Point2D,
    output_cache: Vec<isize>
}

impl ArcadeCabinet {
    fn new() -> ArcadeCabinet {
        let screen = HashMap::new();
        let score = 0;
        let ball = Point2D(0, 0);
        let paddle = Point2D(0, 0);
        let output_cache = Vec::with_capacity(3);
        ArcadeCabinet { screen, score, ball, paddle, output_cache }
    }
}

impl InputProvider for ArcadeCabinet {
    fn get(&mut self) -> IntcodeResult<Option<isize>> {
        Ok(Some((self.ball.0 - self.paddle.0).signum()))
    }
}

impl OutputHandler for ArcadeCabinet {
    fn push(&mut self, v: isize) -> IntcodeResult<()> {
        self.output_cache.push(v);

        if self.output_cache.len() == 3 {
            let x = self.output_cache[0];
            let y = self.output_cache[1];
            let z = self.output_cache[2];

            if (x, y) == (-1, 0) {
                self.score = z;
            }
            else {
                let object = GameObject::from_id(z);
                let point = Point2D(x, y);

                if object == Ball {
                    self.ball = point;
                }

                if object == HorizontalPaddle {
                    self.paddle = point;
                }

                self.screen.insert(Point2D(x, y), object);
            }

            self.output_cache.clear();
        }

        Ok(())
    }
}

fn part1(input: &str) -> IntcodeResult<usize> {
    let cabinet = ArcadeCabinet::new();
    let mut system = System::parse(input, cabinet)?;
    system.run()?;

    let result = system.io.screen.values()
        .filter(|&&v| v == Block)
        .count();

    Ok(result)
}

fn part2(input: &str) -> IntcodeResult<isize> {
    let cabinet = ArcadeCabinet::new();
    let mut system = System::parse(input, cabinet)?;
    system.cpu.memory.set(0, 2);
    system.run()?;
    Ok(system.io.score)
}

build_main_res!("day13.txt", "Part 1" => part1, "Part 2" => part2);