use adventofcode2019::build_main;

fn part1(input: &str) -> usize {
    input.lines()
        .map(|line| line.parse::<usize>().unwrap())
        .map(|mass| mass / 3 - 2)
        .sum::<usize>()
}

fn total_fuel(mut mass: usize) -> usize {
    let mut total = 0;

    while mass >= 9 {
        mass = (mass / 3) - 2;
        total += mass;
    }

    total
}

fn part2(input: &str) -> usize {
    input.lines()
        .map(|line| line.parse::<usize>().unwrap())
        .map(total_fuel)
        .sum::<usize>()
}

build_main!("day01.txt", "Part 1" => part1, "Part 2" => part2);