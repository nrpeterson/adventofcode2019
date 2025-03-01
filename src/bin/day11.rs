use std::cmp::{max, min};
use std::collections::HashMap;
use itertools::{chain, Itertools};
use adventofcode2019::build_main;
use adventofcode2019::intcode::Computer;
use adventofcode2019::points::Point2D;

struct Robot {
    position: Point2D,
    direction: Point2D,
    computer: Computer
}

impl Robot {
    fn parse(input: &str) -> Robot {
        let computer = Computer::parse(input, vec![]);
        let position = Point2D(0, 0);
        let direction = Point2D(0, 1);
        Robot { position, direction, computer }
    }
}

struct Scene {
    colors: HashMap<Point2D, isize>,
    robot: Robot
}

impl Scene {
    fn parse(input: &str) -> Scene {
        let colors = HashMap::new();
        let robot = Robot::parse(input);
        Scene { colors, robot }
    }
}

impl Iterator for Scene {
    type Item = Point2D;
    fn next(&mut self) -> Option<Self::Item> {
        let cur_position = self.robot.position;
        let cur_color = self.colors.entry(self.robot.position).or_insert(0);
        self.robot.computer.input(*cur_color);
        *cur_color = self.robot.computer.next_output()?;

        let turn = self.robot.computer.next_output()?;
        let Point2D(x, y) = self.robot.direction;
        self.robot.direction = if turn == 0 { Point2D(-y, x) }
            else { Point2D(y, -x) };

        self.robot.position = self.robot.position + self.robot.direction;

        Some(cur_position)
    }
}

fn part1(input: &str) -> usize {
    let scene = Scene::parse(input);
    scene.unique().count()
}

fn part2(input: &str) -> String {
    let mut scene = Scene::parse(input);
    scene.colors.insert(Point2D(0, 0), 1);

    while let Some(_) = scene.next() { };
    let (x_min, x_max, y_min, y_max) = scene.colors.keys()
        .fold((isize::MAX, isize::MIN, isize::MAX, isize::MIN),
              |(x0, x1, y0, y1), &Point2D(x, y)| {
                  (min(x0, x), max(x1, x), min(y0, y), max(y1, y))
              }
        );

    let mut message = vec![
        vec!['.'; (x_max - x_min + 1) as usize];
        (y_max - y_min + 1) as usize
    ];

    for (Point2D(x, y), color) in scene.colors {
        let i = (y_max - y) as usize;
        let j = (x - x_min) as usize;
        let c = if color == 0 { '.' } else { '#' };
        message[i][j] = c;
    }

    chain!(std::iter::once(vec![]), message.into_iter())
        .map(|row| row.into_iter().collect::<String>())
        .join("\n")
}

build_main!("day11.txt", "Part 1" => part1, "Part 2" => part2);