use adventofcode2019::build_main_res;
use adventofcode2019::intcode::{IntcodeResult, Resettable, Runnable};
use adventofcode2019::intcode::cpu::{parse_code, CPU};
use adventofcode2019::intcode::IntcodeError::LogicError;

fn part1(input: &str) -> IntcodeResult<isize> {
    let program = parse_code(input)?;
    let mut cpu = CPU::new(program);
    cpu.memory.set(1, 12);
    cpu.memory.set(2, 2);
    cpu.run()?;
    Ok(cpu.memory.get(0))
}

fn part2(input: &str) -> IntcodeResult<isize> {
    let program = parse_code(input)?;
    let mut cpu = CPU::new(program);

    for noun in 0..100 {
        for verb in 0..100 {
            cpu.reset();
            cpu.memory.set(1, noun);
            cpu.memory.set(2, verb);
            cpu.run()?;
            if cpu.memory.get(0) == 19690720 {
                return Ok(100 * noun + verb);
            }
        }
    }

    Err(LogicError("No solution found".to_string()))
}

build_main_res!("day02.txt", "Part 1" => part1, "Part 2" => part2);