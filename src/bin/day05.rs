use adventofcode2019::build_main_res;
use adventofcode2019::computer::io::{Bus, Const, Last};
use adventofcode2019::computer::IntcodeError::ExpectedOutput;
use adventofcode2019::computer::{IntcodeResult, Runnable, System};

fn run_with_input(code: &str, n: isize) -> IntcodeResult<isize> {
    let i = Const(n);
    let o = Last(None);
    let io = Bus { input: i, output: o };
    let mut system = System::parse(code, io)?;
    system.run()?;

    system.io.output.0.ok_or(ExpectedOutput)
}

fn part1(input: &str) -> IntcodeResult<isize> {
    run_with_input(input, 1)
}

fn part2(input: &str) -> IntcodeResult<isize> {
    run_with_input(input, 5)
}

build_main_res!("day05.txt", "Part 1" => part1, "Part 2" => part2);