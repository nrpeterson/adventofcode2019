use itertools::Itertools;
use adventofcode2019::build_main;
use adventofcode2019::intcode::Computer;

fn part1(input: &str) -> isize {
    let computer = Computer::parse(input, vec![1]);
    let outputs = computer.output_runner().collect_vec();
    assert_eq!(outputs.len(), 1);
    outputs[0]
}

fn part2(input: &str) -> isize {
    let computer = Computer::parse(input, vec![2]);
    let outputs = computer.output_runner().collect_vec();
    assert_eq!(outputs.len(), 1);
    outputs[0]
}

build_main!("day09.txt", "Part 1" => part1, "Part 2" => part2);

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use adventofcode2019::intcode::Computer;

    #[test]
    fn test1() {
        let input = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
        let instrs = input.split(',')
            .map(|s| s.parse::<isize>().unwrap())
            .collect_vec();

        let mut c = Computer::new(instrs.clone(), vec![]);
        let actual = c.output_runner().collect_vec();

        assert_eq!(instrs, actual);
    }

    #[test]
    fn test2() {
        let input = "1102,34915192,34915192,7,4,7,99,0";
        let mut c = Computer::parse(input, vec![]);
        let output = c.output_runner().next().unwrap();
        assert_eq!(output.ilog10() + 1, 16);
    }

    #[test]
    fn test3() {
        let input = "104,1125899906842624,99";
        let mut c = Computer::parse(input, vec![]);
        let output = c.output_runner().next().unwrap();
        assert_eq!(output, 1125899906842624);
    }
}