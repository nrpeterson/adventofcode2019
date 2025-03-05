use adventofcode2019::intcode::io::{Bus, Last, OutputHandler};
use adventofcode2019::intcode::IntcodeError::{ExpectedOutput, LogicError};
use adventofcode2019::intcode::{IntcodeResult, Runnable, System};
use adventofcode2019::grid::Direction::{Down, Left, Right, Up};
use adventofcode2019::grid::{Direction, Position};
use adventofcode2019::build_main_res;
use itertools::{chain, Itertools};
use std::cmp::min;
use std::collections::{HashSet, VecDeque};
use std::iter::once;

struct Scene {
    scaffolds: HashSet<Position>,
    robot_pos: Position,
    robot_dir: Direction
}

struct SceneBuilder {
    i: usize,
    j: usize,
    scaffolds: HashSet<Position>,
    robot_pos: Option<Position>,
    robot_dir: Option<Direction>
}

impl SceneBuilder {
    fn new() -> SceneBuilder {
        SceneBuilder {
            i: 0,
            j: 0,
            scaffolds: HashSet::new(),
            robot_pos: None,
            robot_dir: None
        }
    }

    fn build(self) -> IntcodeResult<Scene> {
        let robot_pos = self.robot_pos
            .ok_or(LogicError("Expected robot position".to_string()))?;

        let robot_dir = self.robot_dir
            .ok_or(LogicError("Expected robot direction".to_string()))?;

        Ok(Scene { scaffolds: self.scaffolds, robot_pos, robot_dir })
    }
}

impl OutputHandler for SceneBuilder {
    fn push(&mut self, v: isize) -> IntcodeResult<()> {
        let c = (v as u8) as char;
        match c {
            '#' | '^' | '>' | 'v' | '<' =>  { self.scaffolds.insert(Position(self.i, self.j)); },
            _ => ()
        }

        match c {
            '^' | '>' | 'v' | '<' => { self.robot_pos = Some(Position(self.i, self.j)); },
            _ => ()
        }

        match c {
            '^' => { self.robot_dir = Some(Up); },
            '>' => { self.robot_dir = Some(Right); },
            'v' => { self.robot_dir = Some(Down); },
            '<' => { self.robot_dir = Some(Left); }
            _ => ()
        }

        if c == '\n' {
            self.i += 1;
            self.j = 0;
        }
        else {
            self.j += 1;
        }

        Ok(())
    }
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

fn part1(input: &str) -> IntcodeResult<usize> {
    let io = Bus { input: (), output: SceneBuilder::new() };
    let mut system = System::parse(input, io)?;
    system.run()?;
    let scene = system.io.output.build()?;

    let intersections = scene.intersections();

    let result = intersections.into_iter()
        .map(|Position(i, j)| i * j)
        .sum::<usize>();

    Ok(result)
}

fn part2(input: &str) -> IntcodeResult<isize> {
    let io = Bus { input: (), output: SceneBuilder::new() };
    let mut system = System::parse(input, io)?;
    system.run()?;
    let scene = system.io.output.build()?;

    let programs = scene.programs();

    let mut input_chars = VecDeque::new();
    Itertools::intersperse(programs.program.iter().cloned(), ',')
        .for_each(|c| input_chars.push_back(c));

    input_chars.push_back('\n');
    for prog in [&programs.a, &programs.b, &programs.c] {
        let s = Itertools::intersperse(
            prog.as_ref().unwrap().iter()
            .map(|&cmd| {
                match cmd {
                    Command::TurnLeft => "L".to_owned(),
                    Command::TurnRight => "R".to_owned(),
                    Command::Forward(n) => n.to_string()
                }
            }),
            ",".to_string()
        ).collect::<String>();

        input_chars.extend(s.chars());
        input_chars.push_back('\n');
    }

    input_chars.push_back('n');
    input_chars.push_back('\n');

    let output = Last(None);

    let io2 = Bus { input: input_chars, output };

    let mut system2 = System::parse(input, io2)?;
    system2.cpu.memory.set(0, 2);
    system2.run()?;

    system2.io.output.0.ok_or(ExpectedOutput)
}

build_main_res!("day17.txt", "Part 1" => part1, "Part 2" => part2);