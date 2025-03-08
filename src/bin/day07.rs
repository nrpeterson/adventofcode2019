use adventofcode2019::build_main_res;
use adventofcode2019::intcode::cpu::{parse_code, CPU};
use adventofcode2019::intcode::io::IOQueues;
use adventofcode2019::intcode::IntcodeError::LogicError;
use adventofcode2019::intcode::IntcodeState::Halted;
use adventofcode2019::intcode::{IOWrapper, IntcodeResult, Runnable};
use itertools::Itertools;

type System = IOWrapper<IOQueues, CPU>;

fn init_thrusters(program: &Vec<isize>, phases: Vec<isize>) -> IntcodeResult<Vec<System>> {
    phases.into_iter().map(|x| {
        let io = IOQueues::new();
        let cpu = CPU::new(program.clone());
        let mut system = cpu.wrap(io);
        system.accept_input(x)?;
        Ok(system)
    }).collect()
}

fn part1(input: &str) -> IntcodeResult<isize> {
    let ref program = parse_code(input)?;

    let to_thrusters = |phases: Vec<isize>| -> IntcodeResult<isize> {
        let mut systems = init_thrusters(program, phases)?;
        systems[0].accept_input(0)?;

        for i in 0..4 {
            systems[i].run_until_output()?;
            let msg = format!("Expected an output for intcode {i}");
            let output = systems[i].outer.output
                .pop_front()
                .ok_or(LogicError(msg))?;

            systems[i+1].accept_input(output)?;
        }

        systems[4].run_until_output()?;
        systems[4].outer.output.pop_front().ok_or(LogicError("Expected an output".to_string()))
    };

    let results = (0..5).permutations(5)
        .map(to_thrusters)
        .collect::<IntcodeResult<Vec<isize>>>()?;

    results.into_iter()
        .max()
        .ok_or(LogicError("No results found".to_string()))
}

fn part2(input: &str) -> IntcodeResult<isize> {
    let ref program = parse_code(input)?;

    let to_thrusters = |phases: Vec<isize>| -> IntcodeResult<isize> {
        let mut systems = init_thrusters(program, phases)?;
        systems[0].accept_input(0)?;

        let mut last = 0;

        loop {
            let steps = systems.iter_mut()
                .map(|c| c.step())
                .collect::<IntcodeResult<Vec<_>>>()?;

            if steps.iter().all(|s| *s == Halted) { return Ok(last) };

            for i in 0..5 {
                while let Some(o) = systems[i].outer.output.pop_front() {
                    let j = (i + 1) % 5;
                    systems[j].accept_input(o)?;

                    if i == 4 {
                        last = o;
                    }
                }
            }
        }
    };

    let results = (5..10).permutations(5)
        .map(to_thrusters)
        .collect::<IntcodeResult<Vec<isize>>>()?;

    results.into_iter()
        .max()
        .ok_or(LogicError("No results found".to_string()))
}

build_main_res!("day07.txt", "Part 1" => part1, "Part 2" => part2);

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        let examples = [
            ("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0", 43210),
            ("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0", 54321),
            ("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0", 65210)
        ];

        for (program, expected) in examples {
            assert_eq!(part1(program), Ok(expected));
        }

    }

    #[test]
    fn test_part2() {
        let input = "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
        assert_eq!(part2(input), Ok(139629729));

        let input = "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10";
        assert_eq!(part2(input), Ok(18216));
    }
}