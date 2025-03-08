use adventofcode2019::build_main_res;
use adventofcode2019::intcode::{IOWrapper, IntcodeResult, IntcodeState, Runnable};
use adventofcode2019::intcode::cpu::CPU;
use adventofcode2019::intcode::IntcodeError::{ExpectedOutput, LogicError};
use adventofcode2019::intcode::IntcodeState::Continue;
use adventofcode2019::intcode::io::{IProvider, OProvider};

struct IO {
    const_input: isize,
    output: Option<isize>
}

impl IProvider for IO {
    type PInput = isize;
    type RInput = isize;

    fn provide_input<O>(&mut self) -> IntcodeResult<(IntcodeState<O>, Option<isize>)> {
        Ok((Continue, Some(self.const_input)))
    }

    fn receive_input(&mut self, input: isize) -> IntcodeResult<()> {
        Err(LogicError(format!("Didn't expect to receive any input; received {input}")))
    }
}

impl OProvider for IO {
    type POutput = isize;
    type ROutput = isize;

    fn handle_output(&mut self, output: isize) -> IntcodeResult<IntcodeState<isize>> {
        match self.output {
            None => {
                self.output = Some(output);
                Ok(Continue)
            },
            Some(prev) => Err(LogicError(format!("Expected only one output; got {prev} and {output}")))
        }
    }
}

fn part1(input: &str) -> IntcodeResult<isize> {
    let io = IO { const_input: 1, output: None };
    let cpu = CPU::parse(input)?;
    let mut system = cpu.wrap(io);
    system.run()?;
    system.outer.output.ok_or(ExpectedOutput)
}

fn part2(input: &str) -> IntcodeResult<isize> {
    let io = IO { const_input: 2, output: None };
    let cpu = CPU::parse(input)?;
    let mut system = IOWrapper { outer: io, inner: cpu };
    system.run()?;
    system.outer.output.ok_or(ExpectedOutput)
}

build_main_res!("day09.txt", "Part 1" => part1, "Part 2" => part2);