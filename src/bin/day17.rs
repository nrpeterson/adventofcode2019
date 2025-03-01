use adventofcode2019::build_main;
use adventofcode2019::grid::Direction::{Down, Left, Right, Up};
use adventofcode2019::grid::{Direction, Position};
use adventofcode2019::intcode::Computer;
use itertools::{chain, Itertools};
use std::cmp::min;
use std::collections::{HashSet, VecDeque};
use std::iter::once;

struct Scene {
    scaffolds: HashSet<Position>,
    robot_pos: Position,
    robot_dir: Direction
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Step { Forward, TurnLeft, TurnRight }

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Command { Forward(usize), TurnLeft, TurnRight }

#[derive(Clone, Debug)]
struct State {
    a: Option<Vec<Command>>,
    b: Option<Vec<Command>>,
    c: Option<Vec<Command>>,
    program: Vec<char>,
    remaining: Vec<Command>
}

impl Scene {
    fn new(camera_output: Vec<isize>) -> Scene {
        let s = camera_output.into_iter()
            .map(|i| (i as u8) as char)
            .collect::<String>();

        let mut scaffolds = HashSet::new();
        let mut robot_pos_opt = None;
        let mut robot_dir_opt = None;

        for (i, line) in s.lines().enumerate() {
            for (j, c) in line.chars().enumerate() {
                match c {
                    '#' | '^' | '>' | 'v' | '<' =>  { scaffolds.insert(Position(i, j)); },
                    _ => ()
                }

                match c {
                    '^' | '>' | 'v' | '<' => { robot_pos_opt = Some(Position(i, j)); },
                    _ => ()
                }

                match c {
                    '^' => { robot_dir_opt = Some(Up); },
                    '>' => { robot_dir_opt = Some(Right); },
                    'v' => { robot_dir_opt = Some(Down); },
                    '<' => { robot_dir_opt = Some(Left); }
                    _ => ()
                }
            }
        }

        let robot_pos = robot_pos_opt.unwrap();
        let robot_dir = robot_dir_opt.unwrap();

        Scene { scaffolds, robot_pos, robot_dir }
    }

    fn neighbors(&self, &Position(i, j): &Position) -> Vec<Position> {
        let mut result = vec![];

        if i > 0 && self.scaffolds.contains(&Position(i-1, j)) {
            result.push(Position(i-1, j));
        }
        if j > 0 && self.scaffolds.contains(&Position(i, j-1)) {
            result.push(Position(i, j-1));
        }
        if self.scaffolds.contains(&Position(i+1, j)) {
            result.push(Position(i+1, j));
        }
        if self.scaffolds.contains(&Position(i, j+1)) {
            result.push(Position(i, j+1));
        }

        result
    }

    fn intersections(&self) -> Vec<Position> {
        self.scaffolds.iter().cloned()
            .filter(|pos| self.neighbors(pos).len() > 2)
            .collect()
    }

    fn steps(&self) -> Vec<Step> {
        let mut result = vec![];

        let mut pos = self.robot_pos;
        let mut dir = self.robot_dir;

        loop {
            let next = pos.step(dir).filter(|n| self.scaffolds.contains(n));

            if let Some(next_pos) = next {
                result.push(Step::Forward);
                pos = next_pos;
            }
            else {
                let step_and_dir = [
                    (Step::TurnLeft, dir.counterclockwise()), (Step::TurnRight, dir.clockwise())
                ].into_iter()
                    .filter(|&(_, new_dir)| {
                        pos.step(new_dir).is_some_and(|n| self.scaffolds.contains(&n))
                    })
                    .next();

                match step_and_dir {
                    Some((step, new_dir)) => {
                        dir = new_dir;
                        result.push(step);
                    },
                    None => {
                        return result
                    }
                }
            }
        }
    }

    fn commands(&self) -> Vec<Command> {
        let steps = self.steps();
        let mut result = Vec::new();
        let mut i = 0;
        while i < steps.len() {
            match steps[i] {
                Step::TurnLeft => {
                    result.push(Command::TurnLeft);
                    i += 1;
                },
                Step::TurnRight => {
                    result.push(Command::TurnRight);
                    i += 1;
                },
                Step::Forward => {
                    let mut n = 0;
                    while i + n < steps.len() && steps[i + n] == Step::Forward {
                        n += 1;
                    }
                    result.push(Command::Forward(n));
                    i += n;
                }
            }
        }

        result
    }

    fn programs(&self) -> State {
        let mut queue = VecDeque::new();
        queue.push_back(State { a: None, b: None, c: None, program: vec![], remaining: self.commands() });

        while let Some(state) = queue.pop_front() {
            if state.remaining.is_empty() {
                return state;
            }

            for (sub, c) in [(&state.a, 'A'), (&state.b, 'B'), (&state.c, 'C')] {
                if let Some(cmds) = sub {
                    if state.remaining.starts_with(&cmds) {
                        let remaining = state.remaining[cmds.len()..].iter().cloned().collect_vec();
                        let program = chain!(state.program.iter().cloned(), once(c)).collect_vec();
                        let new_state = State { a: state.a.clone(), b: state.b.clone(), c: state.c.clone(), program, remaining };
                        queue.push_back(new_state);
                    }
                }
            }

            if state.a.is_none() {
                for i in 1..(min(11, state.remaining.len())) {
                    let a = state.remaining[..i].iter().cloned().collect_vec();
                    let remaining = state.remaining[i..].iter().cloned().collect_vec();
                    let program = chain!(state.program.iter().cloned(), once('A')).collect_vec();
                    let new_state = State { a: Some(a), b: None, c: None, program, remaining };
                    queue.push_back(new_state);
                }
            }
            else if state.b.is_none() {
                for i in 1..(min(11, state.remaining.len())) {
                    let b = state.remaining[..i].iter().cloned().collect_vec();
                    let remaining = state.remaining[i..].iter().cloned().collect_vec();
                    let program = chain!(state.program.iter().cloned(), once('B')).collect_vec();
                    let new_state = State { a: state.a.clone(), b: Some(b), c: None, program, remaining };
                    queue.push_back(new_state);
                }
            }
            else if state.c.is_none() {
                for i in 1..(min(11, state.remaining.len())) {
                    let c = state.remaining[..i].iter().cloned().collect_vec();
                    let remaining = state.remaining[i..].iter().cloned().collect_vec();
                    let program = chain!(state.program.iter().cloned(), once('C')).collect_vec();
                    let new_state = State { a: state.a.clone(), b: state.b.clone(), c: Some(c), program, remaining };
                    queue.push_back(new_state);
                }
            }
        }

        unreachable!()
    }
}

fn part1(input: &str) -> usize {
    let computer = Computer::parse(input, vec![]);
    let output = computer.output_runner().collect_vec();
    let scene = Scene::new(output);

    let intersections = scene.intersections();

    intersections.into_iter()
        .map(|Position(i, j)| i * j)
        .sum::<usize>()
}

#[allow(unstable_name_collisions)]
fn part2(input: &str) -> isize {
    let computer = Computer::parse(input, vec![]);
    let output = computer.output_runner().collect_vec();
    let scene = Scene::new(output);

    let programs = scene.programs();

    let mut input_chars = vec![];
    programs.program.iter().cloned().intersperse(',').for_each(|c| input_chars.push(c));
    input_chars.push('\n');
    for prog in [&programs.a, &programs.b, &programs.c] {
        let s = prog.as_ref().unwrap().iter()
            .map(|&cmd| {
                match cmd {
                    Command::TurnLeft => "L".to_owned(),
                    Command::TurnRight => "R".to_owned(),
                    Command::Forward(n) => n.to_string()
                }
            })
            .intersperse(",".to_owned())
            .collect::<String>();

        input_chars.extend(s.chars());
        input_chars.push('\n');
    }

    input_chars.push('n');
    input_chars.push('\n');

    let inputs = input_chars.into_iter()
        .map(|c| c as isize)
        .collect_vec();

    let mut computer2 = Computer::parse(input, inputs);
    computer2.memory.set(0, 2);

    let output = computer2.output_runner().collect_vec();
    *output.last().unwrap()
}

build_main!("day17.txt", "Part 1" => part1, "Part 2" => part2);