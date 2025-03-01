use adventofcode2019::build_main;
use adventofcode2019::intcode::Computer;

fn part1(input: &str) -> isize {
    let c = Computer::parse(input, vec![1]);
    c.generic_runner(|c, _| c.outputs.last().map(|&x| x))
        .last()
        .unwrap()
        .unwrap()
}

fn part2(input: &str) -> isize {
    let c = Computer::parse(input, vec![5]);
    c.generic_runner(|c, _| c.outputs.last().map(|&x| x))
        .last()
        .unwrap()
        .unwrap()
}

build_main!("day05.txt", "Part 1" => part1, "Part 2" => part2);