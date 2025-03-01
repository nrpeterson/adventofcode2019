use std::collections::HashMap;
use itertools::repeat_n;
use nom::branch::alt;
use nom::character::complete::{char, newline};
use nom::combinator::{all_consuming, value};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::{pair, separated_pair};
use adventofcode2019::build_main;
use adventofcode2019::parsers::isize_str;
use adventofcode2019::points::{Direction2D, Point2D};
use adventofcode2019::points::Direction2D::{Down, Left, Right, Up};

fn parse_input(input: &str) -> IResult<&str, (Vec<(Direction2D, isize)>, Vec<(Direction2D, isize)>)> {
    fn term(input: &str) -> IResult<&str, (Direction2D, isize)> {
        pair(
            alt((
                value(Right, char('R')),
                value(Left, char('L')),
                value(Down, char('D')),
                value(Up, char('U')),
            )),
            isize_str
        )(input)
    }

    fn line(input: &str) -> IResult<&str, Vec<(Direction2D, isize)>> {
        separated_list1(char(','), term)(input)
    }

    all_consuming(separated_pair(line, newline, line))(input)
}

fn path(steps: Vec<(Direction2D, isize)>) -> HashMap<Point2D, usize> {
    steps.into_iter()
        .flat_map(|(d, n)| repeat_n(d, n as usize))
        .fold(
            (HashMap::from([(Point2D(0, 0), 0)]), 0, Point2D(0, 0)),
            |(mut seen, steps, cur), x| {
                let new_cur = cur + x.to_step();
                seen.entry(new_cur).or_insert(steps + 1);

                (seen, steps + 1, new_cur)
            }
        ).0
}

fn part1(input: &str) -> isize {
    let (a, b) = parse_input(input).unwrap().1;
    let path_a = path(a);
    let path_b = path(b);

    path_a.keys()
        .filter(|&k| path_b.contains_key(k))
        .map(|k| k.l1_norm())
        .filter(|n| *n > 0)
        .min()
        .unwrap()
}

fn part2(input: &str) -> usize {
    let (a, b) = parse_input(input).unwrap().1;
    let path_a = path(a);
    let path_b = path(b);

    path_a.into_iter()
        .filter(|(k, _)| k.l1_norm() > 0)
        .filter_map(|(k, v)| {
            path_b.get(&k).map(|&w| (k, v + w))
        })
        .min_by_key(|&(_, v)| v)
        .unwrap()
        .1
}

build_main!("day03.txt", "Part 1" => part1, "Part 2" => part2);