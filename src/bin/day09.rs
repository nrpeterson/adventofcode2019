use adventofcode2019::build_main_res;
use adventofcode2019::intcode::{IntcodeResult, Runnable, System};
use std::collections::VecDeque;
use adventofcode2019::intcode::io::{Bus, Const};

fn part1(input: &str) -> IntcodeResult<isize> {
    let io = Bus { input: Const(1), output: VecDeque::new() };
    let mut system = System::parse(input, io)?;
    system.run()?;
    assert_eq!(system.io.output.len(), 1);
    Ok(system.io.output[0])
}

fn part2(input: &str) -> IntcodeResult<isize> {
    let io = Bus { input: Const(2), output: VecDeque::new() };
    let mut system = System::parse(input, io)?;
    system.run()?;
    assert_eq!(system.io.output.len(), 1);
    Ok(system.io.output[0])
}

build_main_res!("day09.txt", "Part 1" => part1, "Part 2" => part2);