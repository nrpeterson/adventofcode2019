use nom::character::complete::{char, digit1};
use nom::combinator::{map_res, opt, recognize};
use nom::IResult;
use nom::sequence::pair;

pub fn usize_str(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse::<usize>())(input)
}

pub fn isize_str(input: &str) -> IResult<&str, isize> {
    map_res(
        recognize(pair(opt(char('-')), digit1)),
        |s: &str| s.parse::<isize>()
    )(input)
}