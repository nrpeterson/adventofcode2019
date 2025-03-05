use crate::computer::cpu::{parse_code, CPU};
use crate::computer::io::IOProvider;
use std::collections::VecDeque;

pub mod io;
pub mod cpu;


#[derive(Debug, Eq, PartialEq)]
pub enum IntcodeError {
    BadParameterMode(isize),
    BadOpCode(isize),
    WriteToImmediate,
    ParsingFailure(String),
    LogicError(String),
    ExpectedOutput
}

#[derive(Debug, Eq, PartialEq)]
pub enum State {
    Continue,
    OutputGenerated,
    Halted,
    AwaitingInput
}

pub type IntcodeResult<T> = Result<T, IntcodeError>;

pub trait Runnable {
    fn step(&mut self) -> IntcodeResult<State>;

    fn run(&mut self) -> IntcodeResult<State> {
        loop {
            let state = self.step()?;

            if state == State::Halted || state == State::AwaitingInput {
                return Ok(state);
            }
        }
    }

    fn run_until_output(&mut self) -> IntcodeResult<()> {
        loop {
            match self.step()? {
                State::OutputGenerated => return Ok(()),
                State::AwaitingInput | State::Halted => return Err(IntcodeError::ExpectedOutput),
                _ => ()
            }
        }
    }
}

pub trait Resettable {
    fn reset(&mut self);
}

impl Resettable for VecDeque<isize> {
    fn reset(&mut self) {
        self.clear()
    }
}

pub struct System<T: IOProvider> {
    pub cpu: CPU,
    pub io: T,
}

impl<T: IOProvider> System<T> {
    pub fn new(program: Vec<isize>, io: T) -> System<T> {
        let cpu = CPU::new(program);
        System { cpu, io }
    }

    pub fn parse(code: &str, io: T) -> IntcodeResult<System<T>> {
        parse_code(code).map(|program| System::new(program, io))
    }
}

impl<T: IOProvider> Runnable for System<T> {
    fn step(&mut self) -> IntcodeResult<State> {
        match self.cpu.step()? {
            State::OutputGenerated => {
                while let Some(x) = self.cpu.output_queue.pop_front() {
                    self.io.push(x)?;
                }
                Ok(State::OutputGenerated)
            },
            State::AwaitingInput => {
                match self.io.get()? {
                    Some(x) => {
                        self.cpu.input_queue.push_back(x);
                        self.step()
                    },
                    None => { Ok(State::AwaitingInput) }
                }
            },
            other => Ok(other)
        }
    }
}

impl<T: IOProvider + Resettable> Resettable for System<T> {
    fn reset(&mut self) {
        self.cpu.reset();
        self.io.reset();
    }
}