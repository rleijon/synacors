use std::io::Read;

pub struct VM {
    pub memory: [u16; 32768],
    pub registers: [u16; 8],
    pub stack: Vec<u16>,
    pub pc: usize,
    pub halted: bool,
    pub debug: bool,
}

#[derive(Debug)]
pub enum Op {
    Halt(),
    Set(u16, u16),
    Push(u16),
    Pop(u16),
    Eq(u16, u16, u16),
    Gt(u16, u16, u16),
    Jmp(u16),
    Jt(u16, u16),
    Jf(u16, u16),
    Add(u16, u16, u16),
    Mult(u16, u16, u16),
    Mod(u16, u16, u16),
    And(u16, u16, u16),
    Or(u16, u16, u16),
    Not(u16, u16),
    Rmem(u16, u16),
    Wmem(u16, u16),
    Call(u16),
    Ret(),
    Out(u16),
    In(u16),
    Noop(),
}

impl VM {
    pub fn run(&mut self) {
        while !self.halted {
            self.execute_next();
        }
    }

    pub fn execute_next(&mut self) {
        let op = self.get_op();
        if self.debug {
            println!("i {} -> {:?}", self.pc, &op);
            println!("Registers: {:?}", self.registers);
        }
        self.execute_op(&op);
    }

    fn get_op(&mut self) -> Op {
        let o = self.memory[self.pc];
        let (op, steps) = match o {
            0 => (Op::Halt(), 1),
            1 => (
                Op::Set(self.memory[self.pc + 1], self.memory[self.pc + 2]),
                3,
            ),
            2 => (Op::Push(self.memory[self.pc + 1]), 2),
            3 => (Op::Pop(self.memory[self.pc + 1]), 2),
            4 => (
                Op::Eq(
                    self.memory[self.pc + 1],
                    self.memory[self.pc + 2],
                    self.memory[self.pc + 3],
                ),
                4,
            ),
            5 => (
                Op::Gt(
                    self.memory[self.pc + 1],
                    self.memory[self.pc + 2],
                    self.memory[self.pc + 3],
                ),
                4,
            ),
            6 => (Op::Jmp(self.memory[self.pc + 1]), 2),
            7 => (
                Op::Jt(self.memory[self.pc + 1], self.memory[self.pc + 2]),
                3,
            ),
            8 => (
                Op::Jf(self.memory[self.pc + 1], self.memory[self.pc + 2]),
                3,
            ),
            9 => (
                Op::Add(
                    self.memory[self.pc + 1],
                    self.memory[self.pc + 2],
                    self.memory[self.pc + 3],
                ),
                4,
            ),
            10 => (
                Op::Mult(
                    self.memory[self.pc + 1],
                    self.memory[self.pc + 2],
                    self.memory[self.pc + 3],
                ),
                4,
            ),
            11 => (
                Op::Mod(
                    self.memory[self.pc + 1],
                    self.memory[self.pc + 2],
                    self.memory[self.pc + 3],
                ),
                4,
            ),
            12 => (
                Op::And(
                    self.memory[self.pc + 1],
                    self.memory[self.pc + 2],
                    self.memory[self.pc + 3],
                ),
                4,
            ),
            13 => (
                Op::Or(
                    self.memory[self.pc + 1],
                    self.memory[self.pc + 2],
                    self.memory[self.pc + 3],
                ),
                4,
            ),
            14 => (
                Op::Not(self.memory[self.pc + 1], self.memory[self.pc + 2]),
                3,
            ),
            15 => (
                Op::Rmem(self.memory[self.pc + 1], self.memory[self.pc + 2]),
                3,
            ),
            16 => (
                Op::Wmem(self.memory[self.pc + 1], self.memory[self.pc + 2]),
                3,
            ),
            17 => (Op::Call(self.memory[self.pc + 1]), 2),
            18 => (Op::Ret(), 1),
            19 => (Op::Out(self.memory[self.pc + 1]), 2),
            20 => (Op::In(self.memory[self.pc + 1]), 2),
            21 => (Op::Noop(), 1),
            _ => panic!(
                "Invalid opcode: {} at {}, next -> {}, prev -> {}",
                o,
                self.pc,
                self.memory[self.pc + 1],
                self.memory[self.pc - 1]
            ),
        };
        self.pc += steps;
        op
    }

    fn execute_op(&mut self, op: &Op) {
        match *op {
            Op::Halt() => {
                self.halted = true;
            }
            Op::Set(a, b) => {
                self.registers[self.get_register(a) as usize] = self.get_value(b);
            }
            Op::Push(a) => {
                self.stack.push(self.get_value(a));
            }
            Op::Pop(a) => {
                self.registers[self.get_register(a) as usize] = self.stack.pop().unwrap();
            }
            Op::Eq(a, b, c) => {
                self.registers[self.get_register(a) as usize] =
                    if self.get_value(b) == self.get_value(c) {
                        1
                    } else {
                        0
                    };
            }
            Op::Gt(a, b, c) => {
                self.registers[self.get_register(a) as usize] =
                    if self.get_value(b) > self.get_value(c) {
                        1
                    } else {
                        0
                    };
            }
            Op::Jmp(a) => {
                self.pc = self.get_value(a) as usize;
            }
            Op::Jt(a, b) => {
                if self.get_value(a) != 0 {
                    self.pc = self.get_value(b) as usize;
                }
            }
            Op::Jf(a, b) => {
                if self.get_value(a) == 0 {
                    self.pc = self.get_value(b) as usize;
                }
            }
            Op::Add(a, b, c) => {
                self.registers[self.get_register(a) as usize] =
                    ((self.get_value(b) as u32 + self.get_value(c) as u32) % 32768) as u16;
            }
            Op::Mult(a, b, c) => {
                self.registers[self.get_register(a) as usize] =
                    ((self.get_value(b) as u32 * self.get_value(c) as u32) % 32768) as u16;
            }
            Op::Mod(a, b, c) => {
                self.registers[self.get_register(a) as usize] =
                    self.get_value(b) % self.get_value(c);
            }
            Op::And(a, b, c) => {
                self.registers[self.get_register(a) as usize] =
                    self.get_value(b) & self.get_value(c);
            }
            Op::Or(a, b, c) => {
                self.registers[self.get_register(a) as usize] =
                    self.get_value(b) | self.get_value(c);
            }
            Op::Not(a, b) => {
                self.registers[self.get_register(a) as usize] = !self.get_value(b) & 0x7FFF;
            }
            Op::Rmem(a, b) => {
                self.registers[self.get_register(a) as usize] =
                    self.memory[self.get_value(b) as usize];
            }
            Op::Wmem(a, b) => {
                self.memory[self.get_value(a) as usize] = self.get_value(b);
            }
            Op::Call(a) => {
                self.stack.push(self.pc as u16);
                self.pc = self.get_value(a) as usize;
            }
            Op::Ret() => {
                self.pc = self.stack.pop().unwrap() as usize;
            }
            Op::Out(a) => {
                print!("{}", self.get_value(a) as u8 as char);
            }
            Op::In(a) => {
                let mut input = [0u8; 1];
                std::io::stdin().read(&mut input).unwrap();
                self.registers[self.get_register(a) as usize] = input[0] as u16;
            }
            Op::Noop() => {
                // do nothing
            }
        }
    }

    fn get_register(&self, reg: u16) -> u16 {
        if reg <= 32767 {
            panic!("Invalid value: {}", reg)
        } else if reg <= 32775 {
            reg - 32768 // register
        } else {
            panic!("Invalid value: {}", reg)
        }
    }

    fn get_value(&self, reg: u16) -> u16 {
        if reg <= 32767 {
            reg // immediate value
        } else if reg <= 32775 {
            self.registers[(reg - 32768) as usize] // register
        } else {
            panic!("Invalid value: {}", reg)
        }
    }

    pub(crate) fn new(file: &[u8]) -> VM {
        let mut vm = VM {
            memory: [0; 32768],
            registers: [0; 8],
            stack: Vec::new(),
            pc: 0,
            halted: false,
            debug: false,
        };
        for i in 0..file.len() / 2 {
            let a = file[i * 2] as u16;
            let b = file[i * 2 + 1] as u16;
            let val = (b << 8) | a;
            vm.memory[i] = val;
        }
        vm
    }
}
