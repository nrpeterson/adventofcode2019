use std::collections::VecDeque;
use adventofcode2019::build_main_res;
use adventofcode2019::intcode::{IntcodeResult, Runnable};
use adventofcode2019::intcode::cpu::CPU;
use adventofcode2019::intcode::IntcodeError::{ExpectedOutput, LogicError};
use adventofcode2019::intcode::io::Bus;

/*
   We want to jump if the space 4 in front of us is ground, and any of the three in front of us
   isn't.
   NOT A J
   NOT B T
   OR T J
   NOT C T
   OR T J
   AND D J
 */
fn part1(input: &str) -> IntcodeResult<isize> {
    let program = "NOT A J\nNOT B T\nOR T J\nNOT C T\nOR T J\nAND D J\nWALK\n";
    let input_queue: VecDeque<char> = program.chars().collect();
    let output_queue: VecDeque<isize> = VecDeque::new();
    let io = Bus { input: input_queue, output: output_queue };
    let cpu = CPU::parse(input)?;
    let mut system = cpu.wrap(io);

    system.run()?;

    let last = *system.outer.output.back().ok_or(ExpectedOutput)?;

    if last <= 255 {
        let msg = system.outer.output.iter()
            .map(|&i| (i as u8) as char)
            .collect::<String>();

        Err(LogicError(msg))
    }
    else {
        Ok(last)
    }
}

/*
   The case we can now avoid is landing somewhere where jumps aren't possible.
   @
   ##..#.##.

   So, jump as before, UNLESS E is false and H is false.

   (E | H) & D & (~A | ~B | ~C)

   But this is hard to encode because we have two compounds.  What is the involvement of E?

   - If E is true, then we don't need to jump right now unless A is false.
   - If E is false and H is false, don't jump now because we'll be stuck.

   So, we can eliminate and just use H, but need to edit hte condition to always jump if the next
   space is empty.

    (H & D & (~B | ~C)) | ~A

   NOT B J
   NOT C T
   OR T J
   AND D J
   AND H J
   NOT A T
   OR T J
 */
fn part2(input: &str) -> IntcodeResult<isize> {
    let program = "NOT B J\nNOT C T\nOR T J\nAND D J\nAND H J\nNOT A T\nOR T J\nRUN\n";
    let input_queue: VecDeque<char> = program.chars().collect();
    let output_queue: VecDeque<isize> = VecDeque::new();
    let io = Bus { input: input_queue, output: output_queue };
    let cpu = CPU::parse(input)?;
    let mut system = cpu.wrap(io);

    system.run()?;

    let last = *system.outer.output.back().ok_or(ExpectedOutput)?;

    if last <= 255 {
        let msg = system.outer.output.iter()
            .map(|&i| (i as u8) as char)
            .collect::<String>();

        println!("{msg}");
        Err(LogicError(msg))
    }
    else {
        Ok(last)
    }
}

build_main_res!("day21.txt", "Part 1" => part1, "Part 2" => part2);