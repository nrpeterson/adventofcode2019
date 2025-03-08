use adventofcode2019::build_main_res;
use adventofcode2019::intcode::cpu::CPU;
use adventofcode2019::intcode::io::{IProvider, OProvider};
use adventofcode2019::intcode::IntcodeError::InputFailure;
use adventofcode2019::intcode::IntcodeState::{AwaitingInput, Continue, OutputGenerated};
use adventofcode2019::intcode::{IntcodeResult, IntcodeState, Runnable};
use std::collections::VecDeque;
use std::io::stdin;

struct Chunker {
    input: VecDeque<char>,
    output: VecDeque<char>
}

impl IProvider for Chunker {
    type PInput = isize;
    type RInput = String;

    fn provide_input<O>(&mut self) -> IntcodeResult<(IntcodeState<O>, Option<isize>)> {
        let result = self.input.pop_front().map(|c| c as isize);
        let st = if result.is_some() { Continue } else { AwaitingInput };
        Ok((st, result))
    }

    fn receive_input(&mut self, input: String) -> IntcodeResult<()> {
        self.input.extend(input.chars());
        Ok(())
    }
}

impl OProvider for Chunker {
    type POutput = String;
    type ROutput = isize;

    fn handle_output(&mut self, output: isize) -> IntcodeResult<IntcodeState<String>> {
        let c = output as u8 as char;
        self.output.push_back(c);

        if c == '\n' {
            let s = String::from_iter(self.output.iter().copied());
            self.output.clear();
            Ok(OutputGenerated(s))
        }
        else {
            Ok(Continue)
        }
    }
}

struct Console { buffer: String }

impl IProvider for Console {
    type PInput = String;
    type RInput = ();

    fn provide_input<O>(&mut self) -> IntcodeResult<(IntcodeState<O>, Option<Self::PInput>)> {
        let mut s = String::new();
        stdin().read_line(&mut s).map_err(|_| InputFailure)?;
        Ok((Continue, Some(s)))
    }

    fn receive_input(&mut self, _: Self::RInput) -> IntcodeResult<()> {
        Err(InputFailure)
    }
}

impl OProvider for Console {
    type POutput = ();
    type ROutput = String;

    fn handle_output(&mut self, output: String) -> IntcodeResult<IntcodeState<()>> {
        print!("{output}");
        Ok(Continue)
    }
}

fn part1(input: &str) -> IntcodeResult<String> {
    let cpu = CPU::parse(input)?;
    let chunker = Chunker { input: VecDeque::new(), output: VecDeque::new() };
    let console = Console { buffer: String::new() };

    let mut system = cpu.wrap(chunker).wrap(console);
    system.run()?;

    Ok("Done".to_string())
}

build_main_res!("day25.txt", "Part 1" => part1);