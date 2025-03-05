use adventofcode2019::computer::io::IOQueues;
use adventofcode2019::computer::{IntcodeResult, Resettable, Runnable, System};
use adventofcode2019::build_main_res;
use itertools::Itertools;


fn part1(input: &str) -> IntcodeResult<isize> {
    let mut system = System::parse(input, IOQueues::new())?;

    let mut get_reading = move |x: isize, y: isize| {
        system.reset();
        system.io.input.extend([x, y]);
        system.run()?;
        Ok(system.io.output.pop_front().unwrap())
    };

    (0..50).cartesian_product(0..50)
        .map(|(x, y)| get_reading(x, y))
        .sum()
}

fn part2(input: &str) -> IntcodeResult<isize> {
    let mut system = System::parse(input, IOQueues::new())?;

    let mut get_reading = move |x: isize, y: isize| {
        system.reset();
        system.io.input.extend([x, y]);
        system.run()?;
        Ok(system.io.output.pop_front().unwrap())
    };

    let mut y = 100;
    let mut x = 0;

    loop {
        while get_reading(x, y)? == 0 {
            x += 1;
        }

        if get_reading(x + 99, y - 99)? == 1 {
            return Ok(10000 * x + y - 99)
        }

        y += 1;
    }
}

build_main_res!("day19.txt", "Part 1" => part1, "Part 2" => part2);