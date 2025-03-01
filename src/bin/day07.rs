use adventofcode2019::build_main;
use adventofcode2019::intcode::{Computer, StepResult};
use itertools::{chain, Itertools};

fn part1(input: &str) -> isize {
    let ref instructions = input.split(',')
        .map(|s| s.parse::<isize>().unwrap())
        .collect_vec();

    let to_thrusters = |a: isize, b: isize, c: isize, d: isize, e: isize| -> isize {
        let c_a = Computer::new(
            instructions.clone(),
            vec![a, 0]
        );
        let c_b = Computer::new(
            instructions.clone(),
            chain!(std::iter::once(b), c_a.output_runner())
        );
        let c_c = Computer::new(
            instructions.clone(),
            chain!(std::iter::once(c), c_b.output_runner())
        );
        let c_d = Computer::new(
            instructions.clone(),
            chain!(std::iter::once(d), c_c.output_runner())
        );
        let c_e = Computer::new(
            instructions.clone(),
            chain!(std::iter::once(e), c_d.output_runner())
        );

        c_e.output_runner().last().unwrap()
    };

    (0..5).permutations(5)
        .map(|v| to_thrusters(v[0], v[1], v[2], v[3], v[4]))
        .max()
        .unwrap()
}

fn part2(input: &str) -> isize {
    let ref instructions = input.split(',')
        .map(|s| s.parse::<isize>().unwrap())
        .collect_vec();

    let to_thrusters = |a: isize, b: isize, c: isize, d: isize, e: isize| -> isize {
        let mut cs = [a, b, c, d, e].map(|x| {
            Computer::new(instructions.clone(), vec![x]).state_runner()
        });

        cs[0].computer.input(0);

        let mut last = 0;

        loop {
            let outs = cs.iter_mut().map(|c| c.next()).collect_vec();
            if outs.iter().all(|out| out.is_none()) {
                return last;
            }

            outs.into_iter().enumerate()
                .for_each(|(i, out)| {
                    if let Some(StepResult::StepComplete { output: Some(o) }) = out {
                        if i == 4 {
                            last = o;
                            cs[0].computer.input(o);
                        }
                        else {
                            cs[i+1].computer.input(o);
                        }
                    }
                });
        }
    };

    (5..10).permutations(5)
        .map(|v| to_thrusters(v[0], v[1], v[2], v[3], v[4]))
        .max()
        .unwrap()
}

build_main!("day07.txt", "Part 1" => part1, "Part 2" => part2);