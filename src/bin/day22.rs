use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::newline;
use nom::combinator::{map, value};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::preceded;
use adventofcode2019::build_main;
use adventofcode2019::math::inverse_mod;
use adventofcode2019::parsers::i128_str;
use crate::Instruction::{Cut, Increment, NewStack};


/*
    Idea: each of these shuffles applies a linear function modulo deck size:

    NewStack: after = (deck_size - 1) - before = -before - 1 (mod deck_size)
    Increment(n): after = n * before (mod deck_size)
    CutPos(n): after = (before - n) (mod deck_size)
    CutNeg(n): after = (before + n) (mod deck_size).

    These can be composed: so for instance, doing Increment(74) -> NewStack is just
    before -> n * before -> -(n * before) - 1 = (-n) * before + (-1).
 */

#[derive(Copy, Clone, Debug)]
struct Polymod {
    a0: i128,
    a1: i128,
    n: i128
}

impl Polymod {
    fn apply(&self, x: i128) -> i128 {
        (((self.a1 * x) % self.n) + self.a0) % self.n
    }
    // that(this)
    fn compose(&self, that: Polymod) -> Polymod {
        /*
            if self(x) = a0 + a1*x and that(x) = b0 + b1*x, then
            that(self(x)) = b0 + b1*(a0 + a1*x) = (b0 + a0*b1) + (a1*b1)x.
         */
        assert_eq!(self.n, that.n);
        let Polymod { a0, a1, n } = *self;
        let Polymod { a0: b0, a1: b1, .. } = that;

        let a0 = ((b1 * a0) % n + b0) % n;
        let a1 = (a1 * b1) % n;
        Polymod { a0, a1, n }
    }

    fn repeat(&self, mut k: i128) -> Polymod {
        let mut result = Polymod { a0: 0, a1: 1, n: self.n };
        let mut p = *self;

        while k > 0 {
            if k % 2 == 1 {
                result = result.compose(p);
            }
            p = p.compose(p);
            k /= 2;
        }

        result
    }

    fn invert(&self) -> Polymod {
        // Have after === a0 + a1 * before
        // after - a0 = a1 * before
        // before = a1^{-1} * (after - a0)

        let a1_inv = inverse_mod(self.a1, self.n);
        let a0 = (-a1_inv * self.a0) % self.n;
        let a1 = a1_inv % self.n;
        Polymod { a0, a1, n: self.n }
    }
}

#[derive(Copy, Clone, Debug)]
enum Instruction {
    NewStack,
    Increment(i128),
    Cut(i128)
}

impl Instruction {
    fn to_polymod(&self, n: i128) -> Polymod {
        match *self {
            NewStack => Polymod { a0: -1, a1: -1, n },
            Increment(i) => Polymod { a0: 0, a1: i % n, n },
            Cut(i) => Polymod { a0: (-i) % n, a1: 1, n }
        }
    }
}

fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    fn instruction(input: &str) -> IResult<&str, Instruction> {
        alt((
            value(NewStack, tag("deal into new stack")),
            map(preceded(tag("deal with increment "), i128_str), Increment),
            map(preceded(tag("cut "), i128_str), Cut),
        ))(input)
    }

    separated_list1(newline, instruction)(input)
}

fn part1(input: &str) -> i128 {
    let instructions = parse(input).unwrap().1;

    let base = Polymod { a0: 0, a1: 1, n: 10007 };
    let full = instructions.into_iter()
        .fold(base, |cur, instr| cur.compose(instr.to_polymod(10007)));

    full.apply(2019)
}

fn part2(input: &str) -> i128 {
    let instructions = parse(input).unwrap().1;

    let n = 119315717514047;

    let base = Polymod { a0: 0, a1: 1, n };
    let full = instructions.into_iter()
        .fold(base, |cur, instr| cur.compose(instr.to_polymod(n)));

    let repeated = full.repeat(101741582076661);
    let inv = repeated.invert();

    inv.apply(2020) % n
}

build_main!("day22.txt", "Part 1" => part1, "Part 2" => part2);