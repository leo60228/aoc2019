use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Argument {
    Address(usize),
    RelativeAddress(isize),
    RelativeImmediate(isize),
    Immediate(isize),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Instruction {
    pub opcode: u8,
    pub arguments: Vec<Argument>,
    pub adj: isize,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mode = self
            .arguments
            .iter()
            .enumerate()
            .map(|(i, a)| {
                10isize.pow((i + 2) as _)
                    * match a {
                        Argument::Address(_) => 0,
                        Argument::RelativeAddress(_) => 0,
                        Argument::Immediate(_) => 1,
                        Argument::RelativeImmediate(_) => 1,
                    }
            })
            .sum::<isize>();
        let instr: isize = mode + isize::from(self.opcode);
        write!(f, "{}", instr)?;
        for &argument in &self.arguments {
            write!(
                f,
                ",{}",
                match argument {
                    Argument::Address(addr) => addr as isize,
                    Argument::RelativeAddress(_) => return Err(fmt::Error),
                    Argument::Immediate(imm) => imm,
                    Argument::RelativeImmediate(_) => return Err(fmt::Error),
                }
            )?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Instructions {
    pub base: usize,
    pub instrs: Vec<Instruction>,
}

impl Instructions {
    pub fn relocate(&mut self) {
        let mut ip = self.base;
        for instr in &mut self.instrs {
            for arg in &mut instr.arguments {
                match arg {
                    Argument::RelativeAddress(addr) => {
                        *arg = Argument::Address(((ip as isize) + *addr + instr.adj) as usize)
                    }
                    Argument::RelativeImmediate(addr) => {
                        *arg = Argument::Immediate((ip as isize) + *addr + instr.adj)
                    }
                    _ => {}
                }
            }
            ip += instr.arguments.len() + 1;
        }
    }

    pub fn join(&mut self, rhs: &Instructions) {
        self.instrs.extend_from_slice(&rhs.instrs);
    }
}

impl fmt::Display for Instructions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut comma = false;
        for instr in &self.instrs {
            if comma {
                write!(f, ",")?;
            } else {
                comma = true;
            }

            write!(f, "{}", instr)?;
        }
        Ok(())
    }
}

fn main() {
    let mut instructions = Instructions {
        base: 0,
        instrs: vec![],
    };

    instructions.join(&ge(
        Argument::Immediate(2),
        Argument::Immediate(1),
        Argument::RelativeAddress(15),
    ));

    instructions.join(&Instructions {
        base: 0,
        instrs: vec![
            Instruction {
                opcode: 5,
                arguments: vec![Argument::RelativeAddress(0), Argument::RelativeImmediate(4)],
                adj: 0,
            },
            Instruction {
                opcode: 0,
                arguments: vec![],
                adj: 0,
            },
            Instruction {
                opcode: 4,
                arguments: vec![Argument::RelativeAddress(-1)],
                adj: 0,
            },
            Instruction {
                opcode: 99,
                arguments: vec![],
                adj: 0,
            },
        ],
    });

    instructions.relocate();

    println!(
        "{}: {}",
        instructions
            .instrs
            .iter()
            .map(|x| 1 + x.arguments.len())
            .sum::<usize>(),
        instructions,
    );
}

pub fn ge(lhs: Argument, rhs: Argument, out: Argument) -> Instructions {
    Instructions {
        base: 0,
        instrs: vec![
            Instruction {
                opcode: 7,
                arguments: vec![lhs, rhs, out],
                adj: 0,
            },
            Instruction {
                opcode: 2,
                arguments: vec![out, Argument::Immediate(-1), out],
                adj: -4,
            },
            Instruction {
                opcode: 1,
                arguments: vec![out, Argument::Immediate(1), out],
                adj: -8,
            },
        ],
    }
}
