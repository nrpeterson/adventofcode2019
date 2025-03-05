use std::collections::VecDeque;
use crate::computer::IntcodeError::LogicError;
use crate::computer::{IntcodeResult, Resettable};

pub trait InputProvider {
    fn get(&mut self) -> IntcodeResult<Option<isize>>;
}

pub trait OutputHandler {
    fn push(&mut self, v: isize) -> IntcodeResult<()>;
}

pub trait IOProvider: InputProvider + OutputHandler { }

impl<T: InputProvider + OutputHandler> IOProvider for T { }

pub struct Bus<I: InputProvider, O: OutputHandler> {
    pub input: I,
    pub output: O
}

impl<I: InputProvider, O: OutputHandler> InputProvider for Bus<I, O> {
    fn get(&mut self) -> IntcodeResult<Option<isize>> {
        self.input.get()
    }
}

impl<I: InputProvider, O: OutputHandler> OutputHandler for Bus<I, O> {
    fn push(&mut self, v: isize) -> IntcodeResult<()> {
        self.output.push(v)
    }
}

impl<I, O> Resettable for Bus<I, O>
where I: InputProvider + Resettable, O: OutputHandler + Resettable {
    fn reset(&mut self) {
        self.input.reset();
        self.output.reset();
    }
}

pub struct IOQueues { pub input: VecDeque<isize>, pub output: VecDeque<isize> }

impl IOQueues {
    pub fn new() -> IOQueues {
        IOQueues { input: VecDeque::new(), output: VecDeque::new() }
    }
}

impl InputProvider for IOQueues {
    fn get(&mut self) -> IntcodeResult<Option<isize>> {
        Ok(self.input.pop_front())
    }
}

impl OutputHandler for IOQueues {
    fn push(&mut self, v: isize) -> IntcodeResult<()> {
        Ok(self.output.push_back(v))
    }
}

impl Resettable for IOQueues {
    fn reset(&mut self) {
        self.input.clear();
        self.output.clear();
    }
}

impl InputProvider for () {
    fn get(&mut self) -> IntcodeResult<Option<isize>> {
        Err(LogicError("Asked for input with no input peripheral".to_string()))
    }
}

impl Resettable for () {
    fn reset(&mut self) {
        ()
    }
}

impl InputProvider for VecDeque<isize> {
    fn get(&mut self) -> IntcodeResult<Option<isize>> {
        Ok(self.pop_front())
    }
}

impl OutputHandler for VecDeque<isize> {
    fn push(&mut self, value: isize) -> IntcodeResult<()> {
        Ok(self.push_back(value))
    }
}

impl InputProvider for VecDeque<char> {
    fn get(&mut self) -> IntcodeResult<Option<isize>> {
        let result = self.pop_front()
            .map(|c| c as isize);

        Ok(result)
    }
}

impl OutputHandler for VecDeque<char> {
    fn push(&mut self, value: isize) -> IntcodeResult<()> {
        self.push_back((value as u8) as char);
        Ok(())
    }
}

pub struct Const(pub isize);

impl InputProvider for Const {
    fn get(&mut self) -> IntcodeResult<Option<isize>> {
        Ok(Some(self.0))
    }
}

impl Resettable for Const {
    fn reset(&mut self) { () }
}

pub struct Last(pub Option<isize>);

impl OutputHandler for Last {
    fn push(&mut self, v: isize) -> IntcodeResult<()> {
        self.0 = Some(v);

        Ok(())
    }
}