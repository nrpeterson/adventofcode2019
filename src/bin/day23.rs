use adventofcode2019::build_main_res;
use adventofcode2019::intcode::cpu::{parse_code, CPU};
use adventofcode2019::intcode::io::{IProvider, OProvider};
use adventofcode2019::intcode::IntcodeError::LogicError;
use adventofcode2019::intcode::IntcodeState::{Continue, OutputGenerated};
use adventofcode2019::intcode::{IOWrapper, IntcodeResult, IntcodeState, Runnable};
use itertools::Itertools;
use std::collections::VecDeque;

struct NICWrapper {
    packets: VecDeque<(isize, isize)>,
    started: bool,
    ip: Option<isize>,
    num_neg_ones: usize,
    output: Vec<isize>
}

impl NICWrapper {
    fn new(ip: isize) -> NICWrapper {
        let packets = VecDeque::new();
        let started = false;
        let ip = Some(ip);
        let num_neg_ones = 0;
        let output = Vec::new();
        NICWrapper { packets, started, ip, num_neg_ones, output }
    }

    fn is_idle(&self) -> bool {
        self.packets.is_empty() && self.num_neg_ones >= 10
    }
}

impl IProvider for NICWrapper {
    type PInput = isize;
    type RInput = (isize, isize);

    fn provide_input<O>(&mut self) -> IntcodeResult<(IntcodeState<O>, Option<isize>)> {
        if let Some(ip) = self.ip {
            self.ip = None;
            return Ok((Continue, Some(ip)))
        }

        if let Some(&(x, y)) = self.packets.get(0) {
            self.num_neg_ones = 0;
            if self.started {
                self.started = false;
                self.packets.pop_front();
                Ok((Continue, Some(y)))
            }
            else {
                self.started = true;
                Ok((Continue, Some(x)))
            }
        }
        else {
            self.num_neg_ones += 1;
            Ok((Continue, Some(-1)))
        }
    }

    fn receive_input(&mut self, input: (isize, isize)) -> IntcodeResult<()> {
        self.packets.push_back(input);
        Ok(())
    }
}

impl OProvider for NICWrapper {
    type POutput = (usize, (isize, isize));
    type ROutput = isize;

    fn handle_output(&mut self, output: isize) -> IntcodeResult<IntcodeState<(usize, (isize, isize))>> {
        self.output.push(output);

        if self.output.len() == 3 {
            let dest = self.output[0] as usize;
            let x = self.output[1];
            let y = self.output[2];

            self.output.clear();
            Ok(OutputGenerated((dest, (x, y))))
        }
        else {
            Ok(Continue)
        }
    }
}

type System = IOWrapper<NICWrapper, CPU>;

fn nics(code: &str) -> IntcodeResult<Vec<System>> {
    let ref program = parse_code(code)?;

    let nics = (0..50).map(|ip| {
        let io = NICWrapper::new(ip);
        let cpu = CPU::new(program.clone());
        cpu.wrap(io)
    }).collect_vec();

    Ok(nics)
}

fn part1(input: &str) -> IntcodeResult<isize> {
    let mut nics = nics(input)?;

    loop {
        for i in 0..50 {
            match nics[i].step()? {
                OutputGenerated((dest, (x, y))) => {
                    if dest == 255 {
                        return Ok(y)
                    }
                    else {
                        nics[dest].accept_input((x, y))?;
                    }
                },
                _ => ()
            }
        }
    }
}

fn part2(input: &str) -> IntcodeResult<isize> {
    let mut nat_memory = None;
    let mut nat_prev_y = None;

    let mut nics = nics(input)?;

    loop {
        for i in 0..50 {
            match nics[i].step()? {
                OutputGenerated((dest, (x, y))) => {
                    if dest == 255 {
                        nat_memory = Some((x, y));
                    }
                    else {
                        nics[dest].accept_input((x, y))?;
                    }
                },
                _ => ()
            }
        }

        if nics.iter().all(|nic| nic.outer.is_idle()) {
            let (x, y) = nat_memory.ok_or(LogicError("Fired NAT without packet".to_string()))?;
            if let Some(prev_y) = nat_prev_y {
                if prev_y == y {
                    return Ok(y)
                }
            }
            nat_prev_y = Some(y);
            nics[0].accept_input((x, y))?;
        }
    }
}

build_main_res!("day23.txt", "Part 1" => part1, "Part 2" => part2);