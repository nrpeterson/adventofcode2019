use crate::intcode::IntcodeState::*;
use crate::intcode::io::{IProvider, OProvider};

pub mod io;
pub mod cpu;


#[derive(Debug, Eq, PartialEq)]
pub enum IntcodeError {
    BadParameterMode(isize),
    BadOpCode(isize),
    WriteToImmediate,
    ParsingFailure(String),
    LogicError(String),
    ExpectedOutput,
    InputFailure
}

#[derive(Debug, Eq, PartialEq)]
pub enum IntcodeState<T> {
    Continue,
    OutputGenerated(T),
    Halted,
    AwaitingInput
}

pub type IntcodeResult<T> = Result<T, IntcodeError>;

pub trait Runnable: Sized {
    type Input;
    type Output;

    fn accept_input(&mut self, input: Self::Input) -> IntcodeResult<()>;

    fn step(&mut self) -> IntcodeResult<IntcodeState<Self::Output>>;

    fn run(&mut self) -> IntcodeResult<IntcodeState<Self::Output>> {
        loop {
            let state = self.step()?;

            match &state {
                Halted | AwaitingInput => return Ok(state),
                _ => continue
            }
        }
    }

    fn run_until_output(&mut self) -> IntcodeResult<Self::Output> {
        loop {
            match self.step()? {
                OutputGenerated(o) => return Ok(o),
                AwaitingInput | IntcodeState::Halted => return Err(IntcodeError::ExpectedOutput),
                _ => ()
            }
        }
    }

    fn wrap<IO>(self, io: IO) -> IOWrapper<IO, Self>
    where IO: IProvider<PInput=Self::Input> + OProvider<ROutput=Self::Output> {
        IOWrapper { outer: io, inner: self }
    }
}

pub trait Resettable {
    fn reset(&mut self);
}

pub struct IOWrapper<Outer, Inner> {
    pub outer: Outer,
    pub inner: Inner
}

impl<Outer, Inner> Runnable for IOWrapper<Outer, Inner>
where Outer: IProvider + OProvider, Inner: Runnable<Input=Outer::PInput, Output=Outer::ROutput> {
    type Input = Outer::RInput;
    type Output = Outer::POutput;

    fn accept_input(&mut self, input: Self::Input) -> IntcodeResult<()> {
        self.outer.receive_input(input)
    }

    fn step(&mut self) -> IntcodeResult<IntcodeState<Self::Output>> {
        match self.inner.step()? {
            OutputGenerated(o) => {
                self.outer.handle_output(o)
            },
            AwaitingInput => {
                let (new_state, output) = self.outer.provide_input()?;
                if let Some(o) = output {
                    self.inner.accept_input(o)?;
                }
                Ok(new_state)
            },
            Halted => Ok(Halted),
            Continue => Ok(Continue)
        }
    }
}

impl<Outer, Inner> Resettable for IOWrapper<Outer, Inner>
where Outer: Resettable, Inner: Resettable {
    fn reset(&mut self) {
        self.outer.reset();
        self.inner.reset();
    }
}