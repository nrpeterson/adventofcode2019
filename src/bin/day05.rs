use adventofcode2019::build_main_res;
use adventofcode2019::intcode::cpu::CPU;
use adventofcode2019::intcode::io::{Bus, ConstInput, Last};
use adventofcode2019::intcode::IntcodeError::ExpectedOutput;
use adventofcode2019::intcode::{IntcodeResult, Runnable};

fn run_with_input(code: &str, n: isize) -> IntcodeResult<isize> {
    let io = Bus { input: ConstInput(n), output: Last(None) };
    let cpu = CPU::parse(code)?;
    let mut system = cpu.wrap(io);
    system.run()?;

    system.outer.output.0.ok_or(ExpectedOutput)
}

fn part1(input: &str) -> IntcodeResult<isize> {
    run_with_input(input, 1)
}

fn part2(input: &str) -> IntcodeResult<isize> {
    run_with_input(input, 5)
}

build_main_res!("day05.txt", "Part 1" => part1, "Part 2" => part2);