use std::collections::{HashMap, HashSet, VecDeque};
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, newline, space1};
use nom::combinator::{all_consuming, map};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use adventofcode2019::build_main;
use adventofcode2019::parsers::usize_str;

#[derive(Copy, Clone)]
struct Substance {
    amount: usize,
    kind: &'static str
}

fn parse_input(input: &'static str) -> IResult<&'static str, HashMap<&'static str, (usize, Vec<Substance>)>> {
    fn substance(input: &'static str) -> IResult<&'static str, Substance> {
        map(
            separated_pair(usize_str, space1, alpha1),
            |(amount, kind)| Substance { amount, kind }
        )(input)
    }

    fn reaction(input: &'static str) -> IResult<&'static str, (&'static str, (usize, Vec<Substance>))> {
        map(
            separated_pair(
                separated_list1(tag(", "), substance),
                tag(" => "),
                substance
            ),
            |(inputs, output)| (output.kind, (output.amount, inputs))
        )(input)
    }

    map(
        all_consuming(separated_list1(newline, reaction)),
        |v| v.into_iter().collect()
    )(input)
}

struct Science {
    reactions: HashMap<&'static str, (usize, Vec<Substance>)>,
    order: Vec<&'static str>,
    indices: HashMap<&'static str, usize>
}

impl Science {
    fn parse(input: &'static str) -> Science {
        let reactions = parse_input(input).unwrap().1;

        let mut simple: HashMap<&str, HashSet<&str>> = reactions.iter()
            .map(|(&k, (_, v))| {
                (k, v.iter().map(|s| s.kind).collect::<HashSet<&str>>())
            })
            .collect();

        let mut order = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back("ORE");

        while let Some(s) = queue.pop_front() {
            order.push(s);
            simple.iter_mut().for_each(|(&k, v)| {
                if v.remove(s) {
                    if v.is_empty() {
                        queue.push_back(k);
                    }
                }
            })
        }

        order.reverse();

        let indices: HashMap<&str, usize> = order.iter().enumerate()
            .map(|(i, &s)| (s, i))
            .collect();

        Science { reactions, order, indices }
    }
    
    fn ore_for_fuel(&self, amount: usize) -> usize {
        let mut amounts = vec![0usize; self.order.len()];

        amounts[0] = amount;

        loop {
            let i = (0..amounts.len()).find(|&i| amounts[i] > 0).unwrap();

            if i == amounts.len() - 1 {
                break;
            }

            let amount = amounts[i];
            let substance = self.order[i];

            let (amount_per, r) = &self.reactions[&substance];
            let amount_per = *amount_per;

            let num_reactions = amount.div_ceil(amount_per);

            for s in r.iter() {
                let j = self.indices[&s.kind];
                amounts[j] += num_reactions * s.amount;
            }

            amounts[i] = 0;
        }

        *amounts.last().unwrap()
    }
}

fn part1(input: &'static str) -> usize {
    let science = Science::parse(input);
    science.ore_for_fuel(1)
}

fn part2(input: &'static str) -> usize {
    let science = Science::parse(input);

    let mut low = science.ore_for_fuel(1);
    let mut high = 1000000000000;

    let t = 1000000000000;

    while low < high {
        let mid = low + (high - low) / 2;

        let ore_for_mid = science.ore_for_fuel(mid);

        if ore_for_mid <= t && science.ore_for_fuel(mid + 1) > t {
            low = mid;
            high = mid;
        }
        else if ore_for_mid <= t {
            low = mid + 1;
        }
        else {
            high = mid - 1;
        }
    }

    low
}

build_main!("day14.txt", "Part 1" => part1, "Part 2" => part2);