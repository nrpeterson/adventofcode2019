use crate::intcode::cpu::Instruction::*;
use crate::intcode::IntcodeError::{InputFailure, ParsingFailure, WriteToImmediate};
use crate::intcode::IntcodeState::{AwaitingInput, Continue, Halted, OutputGenerated};
use crate::intcode::{IntcodeError, IntcodeResult, IntcodeState, Resettable, Runnable};

enum Instruction {
    Add(Parameter, Parameter, Parameter),
    Multiply(Parameter, Parameter, Parameter),
    Input(Parameter),
    Output(Parameter),
    JumpIfTrue(Parameter, Parameter),
    JumpIfFalse(Parameter, Parameter),
    LessThan(Parameter, Parameter, Parameter),
    Equal(Parameter, Parameter, Parameter),
    RelativeBaseOffset(Parameter),
    Done
}

enum Parameter {
    Position(isize),
    Immediate(isize),
    Relative(isize)
}

pub struct Memory {
    rom: Vec<isize>,
    pub ram: Vec<isize>
}

impl Memory {
    fn new(rom: Vec<isize>) -> Memory {
        let ram = rom.clone();
        Memory { rom, ram }
    }

    pub fn get(&self, address: isize) -> isize {
        let i = address as usize;
        if i >= self.ram.len() {
            0
        }
        else {
            self.ram[i]
        }
    }

    pub fn set(&mut self, address: isize, value: isize) {
        let i = address as usize;
        if i >= self.ram.len() {
            self.ram.resize(i + 1, 0);
        }
        self.ram[i] = value;
    }

    pub fn reset(&mut self) {
        self.ram.clear();
        self.ram.extend_from_slice(&self.rom);
    }
}

pub fn parse_code(code: &str) -> IntcodeResult<Vec<isize>> {
    code.split(',')
        .map(|s| s.parse::<isize>().map_err(|e| ParsingFailure(e.to_string())))
        .collect()
}

pub struct CPU {
    pub memory: Memory,
    pub instr_ptr: isize,
    pub rel_base: isize,
    pub input: Option<isize>
}

impl CPU {
    pub fn new(program: Vec<isize>) -> CPU {
        let memory = Memory::new(program);
        let instr_ptr = 0;
        let rel_base = 0;
        let input = None;

        CPU { memory, instr_ptr, rel_base, input }
    }

    pub fn parse(code: &str) -> IntcodeResult<CPU> {
        let program = parse_code(code)?;
        Ok(CPU::new(program))
    }

    fn param(&self, i: usize) -> IntcodeResult<Parameter> {
        let x = self.memory.get(self.instr_ptr + i as isize);
        let mode = (self.memory.get(self.instr_ptr) / 10isize.pow((i + 1) as u32)) % 10;
        match mode {
            0 => Ok(Parameter::Position(x)),
            1 => Ok(Parameter::Immediate(x)),
            2 => Ok(Parameter::Relative(x)),
            _ => Err(IntcodeError::BadParameterMode(mode))
        }
    }

    fn cur_instr(&self) -> IntcodeResult<Instruction> {
        let instr = self.memory.get(self.instr_ptr);
        let op_code = instr % 100;

        match op_code {
            1 => {
                let p1 = self.param(1)?;
                let p2 = self.param(2)?;
                let p3 = self.param(3)?;
                Ok(Add(p1, p2, p3))
            },
            2 => {
                let p1 = self.param(1)?;
                let p2 = self.param(2)?;
                let p3 = self.param(3)?;
                Ok(Multiply(p1, p2, p3))
            },
            3 => {
                let p1 = self.param(1)?;
                Ok(Input(p1))
            },
            4 => {
                let p1 = self.param(1)?;
                Ok(Output(p1))
            },
            5 => {
                let p1 = self.param(1)?;
                let p2 = self.param(2)?;
                Ok(JumpIfTrue(p1, p2))
            },
            6 => {
                let p1 = self.param(1)?;
                let p2 = self.param(2)?;
                Ok(JumpIfFalse(p1, p2))
            },
            7 => {
                let p1 = self.param(1)?;
                let p2 = self.param(2)?;
                let p3 = self.param(3)?;
                Ok(LessThan(p1, p2, p3))
            },
            8 => {
                let p1 = self.param(1)?;
                let p2 = self.param(2)?;
                let p3 = self.param(3)?;
                Ok(Equal(p1, p2, p3))
            },
            9 => {
                let p1 = self.param(1)?;
                Ok(RelativeBaseOffset(p1))
            },
            99 => Ok(Done),
            _ => Err(IntcodeError::BadOpCode(op_code))
        }
    }

    fn get(&self, param: Parameter) -> IntcodeResult<isize> {
        match param {
            Parameter::Position(addr) => Ok(self.memory.get(addr)),
            Parameter::Immediate(val) => Ok(val),
            Parameter::Relative(offset) => Ok(self.memory.get(self.rel_base + offset))
        }
    }

    fn set(&mut self, param: Parameter, value: isize) -> IntcodeResult<()> {
        match param {
            Parameter::Position(addr) => Ok(self.memory.set(addr, value)),
            Parameter::Immediate(_) => Err(WriteToImmediate),
            Parameter::Relative(offset) => Ok(self.memory.set(self.rel_base + offset, value))
        }
    }
}

impl Resettable for CPU {
    fn reset(&mut self) {
        self.memory.reset();
        self.instr_ptr = 0;
        self.rel_base = 0;
        self.input = None;
    }
}

impl Runnable for CPU {
    type Input = isize;
    type Output = isize;

    fn accept_input(&mut self, input: Self::Input) -> IntcodeResult<()> {
        match self.input {
            None => { self.input = Some(input); Ok(()) },
            Some(_) => { Err(InputFailure) }
        }
    }

    fn step(&mut self) -> IntcodeResult<IntcodeState<isize>> {
        match self.cur_instr()? {
            Add(p1, p2, p3) => {
                let v1 = self.get(p1)?;
                let v2 = self.get(p2)?;
                self.set(p3, v1 + v2)?;
                self.instr_ptr += 4;
                Ok(Continue)
            }
            Multiply(p1, p2, p3) => {
                let v1 = self.get(p1)?;
                let v2 = self.get(p2)?;
                self.set(p3, v1 * v2)?;
                self.instr_ptr += 4;
                Ok(Continue)
            }
            Input(p1) => {
                match self.input {
                    Some(val) => {
                        self.input = None;
                        self.set(p1, val)?;
                        self.instr_ptr += 2;
                        Ok(Continue)
                    },
                    None => Ok(AwaitingInput)
                }
            }
            Output(p1) => {
                let v1 = self.get(p1)?;

                self.instr_ptr += 2;
                Ok(OutputGenerated(v1))
            }
            JumpIfTrue(p1, p2) => {
                let v1 = self.get(p1)?;
                if v1 != 0 {
                    self.instr_ptr = self.get(p2)?;
                }
                else {
                    self.instr_ptr += 3;
                }
                Ok(Continue)
            }
            JumpIfFalse(p1, p2) => {
                let v1 = self.get(p1)?;
                if v1 == 0 {
                    self.instr_ptr = self.get(p2)?;
                }
                else {
                    self.instr_ptr += 3;
                }
                Ok(Continue)
            }
            LessThan(p1, p2, p3) => {
                let v1 = self.get(p1)?;
                let v2 = self.get(p2)?;

                let result = if v1 < v2 { 1 } else { 0 };
                self.set(p3, result)?;
                self.instr_ptr += 4;
                Ok(Continue)
            }
            Equal(p1, p2, p3) => {
                let v1 = self.get(p1)?;
                let v2 = self.get(p2)?;
                let result = if v1 == v2 { 1 } else { 0 };
                self.set(p3, result)?;
                self.instr_ptr += 4;
                Ok(Continue)
            }
            RelativeBaseOffset(p1) => {
                let v1 = self.get(p1)?;
                self.rel_base += v1;
                self.instr_ptr += 2;
                Ok(Continue)
            }
            Done => {
                Ok(Halted)
            }
        }
    }
}