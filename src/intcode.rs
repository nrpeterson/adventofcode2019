use std::collections::{HashMap, VecDeque};
use nom::character::complete::char;
use nom::multi::separated_list1;
use crate::intcode::StepResult::{StepComplete, AwaitingInput};
use crate::parsers::isize_str;

#[derive(Clone, Eq, PartialEq)]
pub struct Memory {
    base: Vec<isize>,
    extra: HashMap<usize, isize>
}

impl Memory {
    fn from(base: Vec<isize>) -> Memory {
        let extra = HashMap::new();
        Memory { base, extra }
    }

    pub fn get(&self, index: usize) -> isize {
        if index < self.base.len() {
            self.base[index]
        }
        else {
            self.extra.get(&index).cloned().unwrap_or(0)
        }
    }

    pub fn set(&mut self, index: usize, value: isize) {
        if index < self.base.len() {
            self.base[index] = value;
        }
        else {
            self.extra.insert(index, value);
        }
    }
}

#[derive(Copy, Clone)]
pub enum StepResult {
    StepComplete { output: Option<isize> },
    AwaitingInput
}

#[derive(Copy, Clone)]
pub enum Interaction {
    AwaitingInput,
    CreatedOutput(isize)
}

#[derive(Clone)]
pub struct Computer {
    pub memory: Memory,
    pub ip: usize,
    pub rel_base: isize,
    pub inputs: VecDeque<isize>,
    pub outputs: Vec<isize>
}

impl Computer {
    fn eval(&self, value: isize, mode: isize) -> isize {
        match mode {
            0 => self.memory.get(value as usize),
            1 => value,
            2 => self.memory.get((self.rel_base + value) as usize),
            _ => panic!("invalid mode {mode}")
        }
    }

    fn set(&mut self, addr: isize, mode: isize, value: isize) {
        let a = match mode {
            0 => addr,
            2 => self.rel_base + addr,
            _ => panic!("invalid mode {mode}")
        };

        self.memory.set(a as usize, value);
    }

    pub fn input(&mut self, value: isize) {
        self.inputs.push_back(value);
    }

    pub fn step(&mut self) -> Option<StepResult> {
        let op = self.memory.get(self.ip);
        let op_code = op % 100;
        let mode1 = (op / 100) % 10;
        let mode2 = (op / 1000) % 10;
        let mode3 = (op / 10000) % 10;

        if op_code == 99 {
            return None;
        }

        match op_code {
            1 => {
                let p1 = self.memory.get(self.ip + 1);
                let p2 = self.memory.get(self.ip + 2);
                let p3 = self.memory.get(self.ip + 3);

                let v1 = self.eval(p1, mode1);
                let v2 = self.eval(p2, mode2);

                let result = v1 + v2;
                self.set(p3, mode3, result);
                self.ip += 4;
                Some(StepComplete { output: None })
            },
            2 => {
                let p1 = self.memory.get(self.ip + 1);
                let p2 = self.memory.get(self.ip + 2);
                let p3 = self.memory.get(self.ip + 3);

                let v1 = self.eval(p1, mode1);
                let v2 = self.eval(p2, mode2);

                let result = v1 * v2;
                self.set(p3, mode3, result);
                self.ip += 4;
                Some(StepComplete { output: None })
            },
            3 => {
                if let Some(i) = self.inputs.pop_front() {
                    let p1 = self.memory.get(self.ip + 1);
                    self.set(p1, mode1, i);
                    self.ip += 2;
                    Some(StepComplete { output: None })
                }
                else {
                    Some(AwaitingInput)
                }
            },
            4 => {
                let value = self.eval(self.memory.get(self.ip + 1), mode1);
                self.outputs.push(value);
                self.ip += 2;
                Some(StepComplete { output: Some(value) })
            },
            5 => {
                let p1 = self.memory.get(self.ip + 1);
                let p2 = self.memory.get(self.ip + 2);

                let test = self.eval(p1, mode1);
                if test != 0 {
                    self.ip = self.eval(p2, mode2) as usize;
                }
                else {
                    self.ip += 3;
                }
                Some(StepComplete { output: None })
            },
            6 => {
                let p1 = self.memory.get(self.ip + 1);
                let p2 = self.memory.get(self.ip + 2);

                let test = self.eval(p1, mode1);
                if test == 0 {
                    self.ip = self.eval(p2, mode2) as usize;
                }
                else {
                    self.ip += 3;
                }
                Some(StepComplete { output: None })
            },
            7 => {
                let p1 = self.memory.get(self.ip + 1);
                let p2 = self.memory.get(self.ip + 2);
                let p3 = self.memory.get(self.ip + 3);

                let result = if self.eval(p1, mode1) < self.eval(p2, mode2) { 1 } else { 0 };
                self.set(p3, mode3, result);
                self.ip += 4;
                Some(StepComplete { output: None })
            },
            8 => {
                let p1 = self.memory.get(self.ip + 1);
                let p2 = self.memory.get(self.ip + 2);
                let p3 = self.memory.get(self.ip + 3);

                let result = if self.eval(p1, mode1) == self.eval(p2, mode2) { 1 } else { 0 };
                self.set(p3, mode3, result);
                self.ip += 4;
                Some(StepComplete { output: None })
            },
            9 => {
                let p1 = self.memory.get(self.ip + 1);
                self.rel_base += self.eval(p1, mode1);
                self.ip += 2;
                Some(StepComplete { output: None })
            }
            _ => panic!("Unknown opcode {op_code}"),
        }
    }

    pub fn next_output(&mut self) -> Option<isize> {
        match self.step()? {
            StepComplete { output: Some(output) } => Some(output),
            AwaitingInput => None,
            _ => self.next_output()
        }
    }

    pub fn next_interaction(&mut self) -> Option<Interaction> {
        match self.step()? {
            StepComplete { output: Some(output) } => Some(Interaction::CreatedOutput(output)),
            AwaitingInput => Some(Interaction::AwaitingInput),
            _ => self.next_interaction()
        }
    }

    pub fn new<V>(memory: Vec<isize>, initial_inputs: V) -> Computer where V: IntoIterator<Item=isize> {
        Computer {
            memory: Memory::from(memory),
            ip: 0,
            rel_base: 0,
            inputs: initial_inputs.into_iter().collect(),
            outputs: vec![]
        }
    }
    pub fn parse<V>(program: &str, initial_inputs: V) -> Computer where V: IntoIterator<Item=isize> {
        let memory = separated_list1(char(','), isize_str)(program).unwrap().1;
        Computer::new(memory, initial_inputs)
    }

    pub fn generic_runner<F, T>(self, f: F) -> GenericRunner<F, T>
        where F: Fn(&Computer, StepResult) -> T {
        GenericRunner { computer: self, f }
    }

    pub fn state_runner(self) -> StateRunner {
        StateRunner { computer: self }
    }

    pub fn output_runner(self) -> impl Iterator<Item=isize> {
        self.generic_runner(|_, step| {
            match step {
                StepComplete { output } => output,
                _ => None
            }
        }).filter_map(|output| output)
    }
}

pub struct StateRunner {
    pub computer: Computer
}

impl Iterator for StateRunner {
    type Item = StepResult;
    fn next(&mut self) -> Option<Self::Item> {
        self.computer.step()
    }
}

pub struct GenericRunner<F, T> where F: Fn(&Computer, StepResult) -> T {
    computer: Computer,
    f: F
}

impl<F, T> Iterator for GenericRunner<F, T> where F: Fn(&Computer, StepResult) -> T {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let step_result = self.computer.step()?;
        let output = (self.f)(&self.computer, step_result);
        Some(output)
    }
}
