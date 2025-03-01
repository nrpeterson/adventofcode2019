use itertools::Itertools;
use adventofcode2019::build_main;
use adventofcode2019::intcode::Computer;

fn part1(input: &str) -> isize {
    let mut computer = Computer::parse(input, vec![]);
    computer.memory.set(1, 12);
    computer.memory.set(2, 2);
    computer.generic_runner(|c, _| c.memory.get(0)).last().unwrap()
}

fn part2(input: &str) -> isize {
    let orig = Computer::parse(input, vec![]);

    let eval = |noun: isize, verb: isize| -> isize {
        let mut computer = orig.clone();
        computer.memory.set(1, noun);
        computer.memory.set(2, verb);

        computer.generic_runner(|c, _| c.memory.get(0)).last().unwrap()
    };

    (0..100).cartesian_product(0..100)
        .map(|(noun, verb)| (100*noun + verb, eval(noun, verb)))
        .find(|(_, v)| *v == 19690720)
        .unwrap()
        .0
}

build_main!("day02.txt", "Part 1" => part1, "Part 2" => part2);