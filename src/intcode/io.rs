use crate::intcode::IntcodeState::{AwaitingInput, Continue, OutputGenerated};
use crate::intcode::{IntcodeResult, IntcodeState, Resettable};
use std::collections::VecDeque;
use crate::intcode::IntcodeError::LogicError;

pub trait IProvider {
    type PInput;
    type RInput;

    fn provide_input<O>(&mut self) -> IntcodeResult<(IntcodeState<O>, Option<Self::PInput>)>;

    fn receive_input(&mut self, input: Self::RInput) -> IntcodeResult<()>;
}

pub trait OProvider {
    type POutput;
    type ROutput;

    fn handle_output(&mut self, output: Self::ROutput) -> IntcodeResult<IntcodeState<Self::POutput>>;
}

pub struct Bus<I, O> { pub input: I, pub output: O }

impl<IP: IProvider, OP> IProvider for Bus<IP, OP> {
    type PInput = IP::PInput;
    type RInput = IP::RInput;

    fn provide_input<O>(&mut self) -> IntcodeResult<(IntcodeState<O>, Option<Self::PInput>)> {
        self.input.provide_input()
    }

    fn receive_input(&mut self, input: Self::RInput) -> IntcodeResult<()> {
        self.input.receive_input(input)
    }
}

impl<IP, OP: OProvider> OProvider for Bus<IP, OP> {
    type POutput = OP::POutput;
    type ROutput = OP::ROutput;

    fn handle_output(&mut self, output: Self::ROutput) -> IntcodeResult<IntcodeState<Self::POutput>> {
        self.output.handle_output(output)
    }
}

impl<IP: Resettable, OP: Resettable> Resettable for Bus<IP, OP> {
    fn reset(&mut self) {
        self.input.reset();
        self.output.reset();
    }
}

pub struct IOQueues {
    pub input: VecDeque<isize>,
    pub output: VecDeque<isize>
}

impl IOQueues {
    pub fn new() -> IOQueues {
        IOQueues { input: VecDeque::new(), output: VecDeque::new() }
    }
}

impl IProvider for IOQueues {
    type PInput = isize;
    type RInput = isize;

    fn provide_input<O>(&mut self) -> IntcodeResult<(IntcodeState<O>, Option<isize>)> {
        match self.input.pop_front() {
            s if s.is_some() => Ok((Continue, s)),
            _ => Ok((AwaitingInput, None))
        }
    }

    fn receive_input(&mut self, input: Self::RInput) -> IntcodeResult<()> {
        self.input.push_back(input);
        Ok(())
    }
}

impl OProvider for IOQueues {
    type POutput = isize;
    type ROutput = isize;

    fn handle_output(&mut self, output: isize) -> IntcodeResult<IntcodeState<isize>> {
        self.output.push_back(output);
        Ok(OutputGenerated(output))
    }
}

impl Resettable for IOQueues {
    fn reset(&mut self) {
        self.input.clear();
        self.output.clear();
    }
}

pub struct ConstInput<T>(pub T);

impl<T: Clone> IProvider for ConstInput<T> {
    type PInput = T;
    type RInput = ();

    fn provide_input<O>(&mut self) -> IntcodeResult<(IntcodeState<O>, Option<T>)> {
        Ok((Continue, Some(self.0.clone())))
    }

    fn receive_input(&mut self, _: Self::RInput) -> IntcodeResult<()> {
        Err(LogicError("Does not accept input".to_string()))
    }
}

pub struct Last<T>(pub Option<T>);

impl<T> OProvider for Last<T> {
    type POutput = ();
    type ROutput = T;

    fn handle_output(&mut self, output: T) -> IntcodeResult<IntcodeState<()>> {
        self.0 = Some(output);
        Ok(Continue)
    }
}

impl IProvider for VecDeque<isize> {
    type PInput = isize;
    type RInput = isize;

    fn provide_input<O>(&mut self) -> IntcodeResult<(IntcodeState<O>, Option<Self::PInput>)> {
        let result = self.pop_front();
        let state = if result.is_some() { Continue } else { AwaitingInput };
        Ok((state, result))
    }

    fn receive_input(&mut self, input: Self::RInput) -> IntcodeResult<()> {
        self.push_back(input);
        Ok(())
    }
}

impl OProvider for VecDeque<isize> {
    type POutput = ();
    type ROutput = isize;

    fn handle_output(&mut self, output: isize) -> IntcodeResult<IntcodeState<()>> {
        self.push_back(output);
        Ok(Continue)
    }
}

impl IProvider for VecDeque<char> {
    type PInput = isize;
    type RInput = char;

    fn provide_input<O>(&mut self) -> IntcodeResult<(IntcodeState<O>, Option<isize>)> {
        let result = self.pop_front()
            .map(|c| c as isize);

        let state = if result.is_some() { Continue } else { AwaitingInput };

        Ok((state, result))
    }

    fn receive_input(&mut self, input: char) -> IntcodeResult<()> {
        self.push_back(input);
        Ok(())
    }
}

impl<T> Resettable for VecDeque<T> {
    fn reset(&mut self) {
        self.clear()
    }
}