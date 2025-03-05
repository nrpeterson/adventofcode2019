use adventofcode2019::build_main_res;
use adventofcode2019::computer::IntcodeError::LogicError;
use adventofcode2019::computer::{IntcodeResult, Runnable, System};
use adventofcode2019::computer::io::{Bus, Const, Last};

fn part1(input: &str) -> IntcodeResult<isize> {
    let i = Const(1);
    let o = Last(None);
    let io = Bus { input: i, output: o };
    let mut system = System::parse(input, io)?;
    system.run()?;

    system.io.output.0.ok_or(LogicError("No output".to_string()))
}

fn part2(input: &str) -> IntcodeResult<isize> {
    let i = Const(5);
    let o = Last(None);
    let io = Bus { input: i, output: o };
    let mut system = System::parse(input, io)?;
    system.run()?;

    system.io.output.0.ok_or(LogicError("No output".to_string()))
}

build_main_res!("day05.txt", "Part 1" => part1, "Part 2" => part2);