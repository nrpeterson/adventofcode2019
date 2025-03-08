use adventofcode2019::build_main_res;
use adventofcode2019::intcode::cpu::CPU;
use adventofcode2019::intcode::io::IOQueues;
use adventofcode2019::intcode::{IOWrapper, IntcodeResult, Resettable, Runnable};
use itertools::Itertools;

type System = IOWrapper<IOQueues, CPU>;

fn get_reading(system: &mut System, x: isize, y: isize) -> IntcodeResult<isize> {
    system.reset();
    system.outer.input.extend([x, y]);
    system.run()?;
    Ok(system.outer.output.pop_front().unwrap())
}

fn part1(input: &str) -> IntcodeResult<isize> {
    let io = IOQueues::new();
    let cpu = CPU::parse(input)?;
    let mut system = cpu.wrap(io);

    (0..50).cartesian_product(0..50)
        .map(|(x, y)| get_reading(&mut system, x, y))
        .sum()
}

fn part2(input: &str) -> IntcodeResult<isize> {
    let io = IOQueues::new();
    let cpu = CPU::parse(input)?;
    let mut system = cpu.wrap(io);

    let mut y = 100;
    let mut x = 0;

    loop {
        while get_reading(&mut system, x, y)? == 0 {
            x += 1;
        }

        if get_reading(&mut system, x + 99, y - 99)? == 1 {
            return Ok(10000 * x + y - 99)
        }

        y += 1;
    }
}

build_main_res!("day19.txt", "Part 1" => part1, "Part 2" => part2);