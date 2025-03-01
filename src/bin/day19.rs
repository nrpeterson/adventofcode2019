use itertools::Itertools;
use adventofcode2019::build_main;
use adventofcode2019::intcode::Computer;

fn get_reading(program: &Vec<isize>, x: isize, y: isize) -> isize {
    Computer::new(program.clone(), vec![x, y])
        .next_output()
        .unwrap()
}

fn part1(input: &str) -> isize {
    let program = input.split(",")
        .map(|s| s.parse::<isize>().unwrap())
        .collect_vec();

    (0..50).cartesian_product(0..50)
        .map(|(x, y)| get_reading(&program, x, y))
        .sum()
}

fn part2(input: &str) -> isize {
    let program = input.split(",")
        .map(|s| s.parse::<isize>().unwrap())
        .collect_vec();

    let mut y = 100;
    let mut x = 0;

    loop {
        while get_reading(&program, x, y) == 0 {
            x += 1;
        }

        if get_reading(&program, x + 99, y - 99) == 1 {
            return 10000 * x + y - 99
        }

        y += 1;
    }
}

build_main!("day19.txt", "Part 1" => part1, "Part 2" => part2);