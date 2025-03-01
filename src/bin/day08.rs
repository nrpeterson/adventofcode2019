use std::collections::HashMap;
use itertools::Itertools;
use adventofcode2019::build_main;

fn counts<I>(it: I) -> HashMap<char, usize> where I: Iterator<Item=char> {
    it.fold(HashMap::new(), |mut acc, ch| {
        *acc.entry(ch).or_insert(0) += 1;
        acc
    })
}

fn part1(input: &str) -> usize {
    input.chars()
        .chunks(25 * 6).into_iter()
        .map(|chunk| counts(chunk))
        .min_by_key(|c| c[&'0'])
        .map(|c| c[&'1'] * c[&'2'])
        .unwrap()
}

fn part2(input: &str) -> &str {
    input.chars()
        .chunks(25 * 6).into_iter()
        .fold(vec!['2'; 25 * 6], |mut acc, chunk| {
            chunk.enumerate()
                .for_each(|(i, c)| {
                    if acc[i] == '2' {
                        acc[i] = c;
                    }
                });
            acc
        })
        .into_iter()
        .map(|c| if c == '0' { ' ' } else { '#' })
        .chunks(25).into_iter()
        .for_each(|chunk| println!("{}", chunk.collect::<String>()));

    "See above"
}

build_main!("day08.txt", "Part 1" => part1, "Part 2" => part2);