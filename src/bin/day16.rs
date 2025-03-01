use itertools::Itertools;
use adventofcode2019::build_main;

struct Pattern {
    term: usize,
    n_rem: usize,
    n: usize
}

impl Pattern {
    fn new(n: usize) -> Pattern {
        Pattern { term: 0, n_rem: n - 1, n }
    }
}

impl Iterator for Pattern {
    type Item = isize;
    fn next(&mut self) -> Option<Self::Item> {
        if self.n_rem == 0 {
            self.term = (self.term + 1) % 4;
            self.n_rem = self.n;
        }

        let elem = match self.term {
            0 | 2 => 0,
            1 => 1,
            3 => -1,
            _ => unreachable!()
        };

        self.n_rem -= 1;
        Some(elem)
    }
}

fn fft(digits: &[isize]) -> Vec<isize> {
    (1..=digits.len()).map(|n| {
        digits.iter().zip(Pattern::new(n))
            .map(|(&d, p)| d * p)
            .sum::<isize>()
            .abs() % 10
    }).collect_vec()
}

fn parse(input: &str) -> Vec<isize> {
    input.chars().map(|c| (c as isize) - ('0' as isize)).collect_vec()
}

fn part1(input: &str) -> isize {
    let mut digits = parse(input);
    (0..100).for_each(|_| digits = fft(&digits));
    digits.into_iter().take(8)
        .fold(0, |acc, d| acc * 10 + d)
}

/*
    Input is 650 digits, so total of 6,500,000 after repeats.
    Input offset is offset := 5,970,157.

    When we consider the digit at index n, our pattern is:
    * n zeroes
    * n+1 ones
    * n+1 zeroes
    * n+1 negative ones
    * n+1 zeroes
    ...

    In particular, if n + (n+1) >= length, we only get:
    * n zeroes,
    * length - n ones

    So if k=length,
    - the last digit (n = k-1) is constant: d_i(k-1)=d_0(k-1) for all i.
    - for n = k - 2: d_i(k-2)=d_{i-1}(k-2)+d_{i-1}(k-1)
                             =d_{i-1}(k-2)+d_i(k-1)

    - for n = k - 3: d_i(k-3)=d_{i-1}(k-3)+d_{i-1}(k-2)+d_{i-1}(k-1)
                             =d_{i-1}(k-3)+d_i(k-2)

    - for n = k - 4: d_i(k-4)=d_{i-1}(k-4)+d_{i-1}(k-3)+d_{i-1}(k-2)+d_{i-1}(k-1)
                             =d_{i-1}(k-4)+d_i(k-3)
    ...

 */
fn part2(input: &str) -> usize {
    let orig_digits = parse(input);
    let orig_len = orig_digits.len();
    let len = 10000 * orig_len;
    let offset = (&orig_digits[0..7]).iter()
        .fold(0usize, |acc, &d| acc * 10 + (d as usize));

    assert!(2*offset + 1 >= len);

    let mut digits = orig_digits.into_iter().cycle().take(len).collect_vec();

    for _ in 0..100 {
        for i in (offset..=len-2).rev() {
            digits[i] = (digits[i] + digits[i+1]) % 10;
        }
    }

    digits[offset..offset+8].iter().fold(0, |acc, &d| acc * 10 + (d as usize))
}

build_main!("day16.txt", "Part 1" => part1, "Part 2" => part2);