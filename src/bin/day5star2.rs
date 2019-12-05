use std::convert::{TryFrom, TryInto};
use std::io::{self, prelude::*};

pub fn exec(memory: &mut [isize]) {
    let mut ip = 0;
    loop {
        let instr = memory[ip];
        let opcode = instr % 100;
        let mode = instr / 100;
        let arity = match opcode {
            1 => {
                let a_mode = mode % 10;
                let b_mode = (mode / 10) % 10;
                let c_mode = (mode / 100) % 10;
                assert_eq!(c_mode, 0);

                let a = memory[ip + 1];
                let b = memory[ip + 2];
                let c = memory[ip + 3];

                let a_val = match a_mode {
                    0 => memory[usize::try_from(a).unwrap()],
                    1 => a,
                    _ => unimplemented!(),
                };
                let b_val = match b_mode {
                    0 => memory[usize::try_from(b).unwrap()],
                    1 => b,
                    _ => unimplemented!(),
                };

                memory[usize::try_from(c).unwrap()] = a_val + b_val;
                3
            }
            2 => {
                let a_mode = mode % 10;
                let b_mode = (mode / 10) % 10;
                let c_mode = (mode / 100) % 10;
                assert_eq!(c_mode, 0);

                let a = memory[ip + 1];
                let b = memory[ip + 2];
                let c = memory[ip + 3];

                let a_val = match a_mode {
                    0 => memory[usize::try_from(a).unwrap()],
                    1 => a,
                    _ => unimplemented!(),
                };
                let b_val = match b_mode {
                    0 => memory[usize::try_from(b).unwrap()],
                    1 => b,
                    _ => unimplemented!(),
                };

                memory[usize::try_from(c).unwrap()] = a_val * b_val;
                3
            }
            3 => {
                let a_mode = mode % 10;
                assert_eq!(a_mode, 0);
                let a = memory[ip + 1];
                memory[usize::try_from(a).unwrap()] = 5; // TODO: stdin
                1
            }
            4 => {
                let a_mode = mode % 10;
                let a = memory[ip + 1];
                let a_val = match a_mode {
                    0 => memory[usize::try_from(a).unwrap()],
                    1 => a,
                    _ => unimplemented!(),
                };
                println!("{}", a_val);
                1
            }
            5 => {
                let a_mode = mode % 10;
                let b_mode = (mode / 10) % 10;

                let a = memory[ip + 1];
                let b = memory[ip + 2];

                let a_val = match a_mode {
                    0 => memory[usize::try_from(a).unwrap()],
                    1 => a,
                    _ => unimplemented!(),
                };
                let b_val = match b_mode {
                    0 => memory[usize::try_from(b).unwrap()],
                    1 => b,
                    _ => unimplemented!(),
                };

                if a_val != 0 {
                    ip = b_val.try_into().unwrap();
                    -1
                } else {
                    2
                }
            }
            6 => {
                let a_mode = mode % 10;
                let b_mode = (mode / 10) % 10;

                let a = memory[ip + 1];
                let b = memory[ip + 2];

                let a_val = match a_mode {
                    0 => memory[usize::try_from(a).unwrap()],
                    1 => a,
                    _ => unimplemented!(),
                };
                let b_val = match b_mode {
                    0 => memory[usize::try_from(b).unwrap()],
                    1 => b,
                    _ => unimplemented!(),
                };

                if a_val == 0 {
                    ip = b_val.try_into().unwrap();
                    -1
                } else {
                    2
                }
            }
            7 => {
                let a_mode = mode % 10;
                let b_mode = (mode / 10) % 10;
                let c_mode = (mode / 100) % 10;
                assert_eq!(c_mode, 0);

                let a = memory[ip + 1];
                let b = memory[ip + 2];
                let c = memory[ip + 3];

                let a_val = match a_mode {
                    0 => memory[usize::try_from(a).unwrap()],
                    1 => a,
                    _ => unimplemented!(),
                };
                let b_val = match b_mode {
                    0 => memory[usize::try_from(b).unwrap()],
                    1 => b,
                    _ => unimplemented!(),
                };

                memory[usize::try_from(c).unwrap()] = if a_val < b_val { 1 } else { 0 };

                3
            }
            8 => {
                let a_mode = mode % 10;
                let b_mode = (mode / 10) % 10;
                let c_mode = (mode / 100) % 10;
                assert_eq!(c_mode, 0);

                let a = memory[ip + 1];
                let b = memory[ip + 2];
                let c = memory[ip + 3];

                let a_val = match a_mode {
                    0 => memory[usize::try_from(a).unwrap()],
                    1 => a,
                    _ => unimplemented!(),
                };
                let b_val = match b_mode {
                    0 => memory[usize::try_from(b).unwrap()],
                    1 => b,
                    _ => unimplemented!(),
                };

                memory[usize::try_from(c).unwrap()] = if a_val == b_val { 1 } else { 0 };

                3
            }
            99 => break,
            unimp => unimplemented!("unimplemented opcode {}", unimp),
        };
        ip += (arity + 1) as usize;
    }
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;

    let mut memory: Vec<isize> = buf
        .trim()
        .split(',')
        .map(str::trim)
        .flat_map(str::parse)
        .collect();

    exec(&mut memory);

    Ok(())
}

#[cfg(test)]
mod test {
    #[test]
    fn exec() {
        use super::exec;
        fn check(inp: &[isize], out: &[isize]) {
            let mut vec: Vec<_> = inp.into();
            exec(&mut vec);
            assert_eq!(vec, out);
        }
        check(&[1, 0, 0, 0, 99], &[2, 0, 0, 0, 99]);
        check(&[2, 3, 0, 3, 99], &[2, 3, 0, 6, 99]);
        check(&[2, 4, 4, 5, 99, 0], &[2, 4, 4, 5, 99, 9801]);
        check(
            &[1, 1, 1, 4, 99, 5, 6, 0, 99],
            &[30, 1, 1, 4, 2, 5, 6, 0, 99],
        );
    }
}
