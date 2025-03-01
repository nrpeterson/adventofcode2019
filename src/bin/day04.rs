use itertools::Itertools;
use nom::character::complete::char;
use nom::IResult;
use nom::sequence::separated_pair;
use adventofcode2019::build_main;
use adventofcode2019::parsers::usize_str;

fn min_nondecr_at_least(mut n: usize) -> Vec<usize> {
    let mut digits = Vec::new();
    while n > 0 {
        digits.push(n % 10);
        n /= 10;
    }

    digits.reverse();

    for i in 0..digits.len() - 1 {
        if digits[i+1] < digits[i] {
            digits[i+1] = digits[i];
        }
    }

    digits
}

fn parse_input(input: &str) -> IResult<&str, (usize, usize)> {
    separated_pair(usize_str, char('-'), usize_str)(input)
}

struct NonDecrSequences {
    digits: Vec<usize>,
    upper_bound: usize
}

impl Iterator for NonDecrSequences {
    type Item = Vec<usize>;
    fn next(&mut self) -> Option<Self::Item> {
        let mut n = 0;
        for &d in self.digits.iter() {
            n = 10 * n + d;
        }

        if n > self.upper_bound { return None; }

        let result = self.digits.clone();

        let mut i = self.digits.len();
        while i > 0 && self.digits[i-1] == 9 {
            i -= 1;
        }

        if i == 0 {
            self.digits.insert(0, 1);
        }
        else {
            self.digits[i-1] += 1;
            for j in i..self.digits.len() {
                self.digits[j] = self.digits[i-1];
            }
        }
        Some(result)
    }
}

impl NonDecrSequences {
    fn new(low: usize, high: usize) -> NonDecrSequences {
        NonDecrSequences { digits: min_nondecr_at_least(low), upper_bound: high }
    }
}

fn contains_pair(v: &Vec<usize>) -> bool {
    v.iter()
        .tuple_windows()
        .any(|(&a, &b)| a == b)
}

fn part1(input: &str) -> usize {
    let (m, n) = parse_input(input).unwrap().1;
    NonDecrSequences::new(m, n).filter(contains_pair).count()
}

fn contains_isolated_pair(v: &Vec<usize>) -> bool {
    if v[0] == v[1] && v[1] != v[2] {
        return true;
    }

    let n = v.len();
    if v[n-1] == v[n-2] && v[n-2] != v[n-3] {
        return true;
    }

    v.iter()
        .tuple_windows()
        .any(|(&a, &b, &c, &d)| {
            b == c && a != b && c != d
        })
}

fn part2(input: &str) -> usize {
    let (m, n) = parse_input(input).unwrap().1;
    NonDecrSequences::new(m, n).filter(contains_isolated_pair).count()
}

build_main!("day04.txt", "Part 1" => part1, "Part 2" => part2);