use std::collections::HashMap;
use adventofcode2019::build_main;
use adventofcode2019::intcode::{Computer, Interaction};
use adventofcode2019::points::Point2D;
use crate::GameObject::{Ball, Block, Empty, HorizontalPaddle, Wall};

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
    computer: Computer,
    score: isize,
    ball: Point2D,
    paddle: Point2D
}

impl ArcadeCabinet {
    fn parse(input: &str) -> ArcadeCabinet {
        let computer = Computer::parse(input, vec![]);
        let screen = HashMap::new();
        let score = 0;
        let ball = Point2D(0, 0);
        let paddle = Point2D(0, 0);
        ArcadeCabinet { screen, computer, score, ball, paddle }
    }

    fn step(&mut self) -> Option<()> {
        match self.computer.next_interaction()? {
            Interaction::AwaitingInput => {
                let dir = (self.ball.0 - self.paddle.0).signum();
                self.computer.input(dir);
            },
            Interaction::CreatedOutput(x) => {
                let y = self.computer.next_output()?;

                if (x, y) == (-1, 0) {
                    self.score = self.computer.next_output()?;
                }
                else {
                    let object_code = self.computer.next_output()?;
                    let object = GameObject::from_id(object_code);
                    let point = Point2D(x, y);

                    if object == Ball {
                        self.ball = point;
                    }

                    if object == HorizontalPaddle {
                        self.paddle = point;
                    }

                    self.screen.insert(Point2D(x, y), object);
                }
            }
        }

        Some(())
    }

    fn run(&mut self) {
        while let Some(_) = self.step() { continue; }
    }
}

fn part1(input: &str) -> usize {
    let mut cabinet = ArcadeCabinet::parse(input);
    cabinet.run();

    cabinet.screen.values()
        .filter(|&&v| v == Block)
        .count()
}

fn part2(input: &str) -> isize {
    let mut cabinet = ArcadeCabinet::parse(input);
    cabinet.computer.memory.set(0, 2);
    cabinet.run();
    cabinet.score
}

build_main!("day13.txt", "Part 1" => part1, "Part 2" => part2);