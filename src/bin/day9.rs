use std::convert::{TryFrom, TryInto};
use std::io::{self, prelude::*};
use std::iter;

pub fn decode<'a>(
    mode: isize,
    mut ip: usize,
    memory: &'a [isize],
    base: isize,
) -> impl Iterator<Item = isize> + 'a {
    let mut idx = 0;
    iter::from_fn(move || {
        ip += 1;
        let arg = memory[ip];
        let arg_mode = (mode / 10isize.pow(idx)) % 10;
        idx += 1;
        Some(match arg_mode {
            0 => memory[usize::try_from(arg).unwrap()],
            1 => arg,
            2 => memory[usize::try_from(arg + base).unwrap()],
            _ => unimplemented!(),
        })
    })
}

pub fn set_argument(
    mode: isize,
    memory: &mut [isize],
    ip: usize,
    base: isize,
    idx: usize,
    value: isize,
) {
    let arg_mode = (mode / 10isize.pow(idx as u32)) % 10;
    let addr = match arg_mode {
        0 => usize::try_from(memory[ip + idx + 1]).unwrap(),
        1 => ip + idx,
        2 => usize::try_from(memory[ip + idx + 1] + base).unwrap(),
        _ => unimplemented!(),
    };
    memory[addr] = value;
}

pub fn exec(memory: &mut [isize]) {
    let mut ip = 0;
    let mut base = 0;
    loop {
        let instr = memory[ip];
        let opcode = instr % 100;
        let mode = instr / 100;
        let mut decoder = decode(mode, ip, &memory, base);
        let arity = match opcode {
            1 => {
                let a = decoder.next().unwrap();
                let b = decoder.next().unwrap();
                drop(decoder);

                set_argument(mode, memory, ip, base, 2, a + b);
                3
            }
            2 => {
                let a = decoder.next().unwrap();
                let b = decoder.next().unwrap();
                drop(decoder);

                set_argument(mode, memory, ip, base, 2, a * b);
                3
            }
            3 => {
                drop(decoder);

                print!("input: ");
                io::stdout().flush().unwrap();
                let mut buf = String::new();
                io::stdin().read_line(&mut buf).unwrap();

                set_argument(mode, memory, ip, base, 0, buf.trim().parse().unwrap());
                1
            }
            4 => {
                let a = decoder.next().unwrap();
                drop(decoder);

                println!("{}", a);
                1
            }
            5 => {
                let a = decoder.next().unwrap();
                let b = decoder.next().unwrap();
                drop(decoder);

                if a != 0 {
                    ip = b.try_into().unwrap();
                    -1
                } else {
                    2
                }
            }
            6 => {
                let a = decoder.next().unwrap();
                let b = decoder.next().unwrap();
                drop(decoder);

                if a == 0 {
                    ip = b.try_into().unwrap();
                    -1
                } else {
                    2
                }
            }
            7 => {
                let a = decoder.next().unwrap();
                let b = decoder.next().unwrap();
                drop(decoder);

                set_argument(mode, memory, ip, base, 2, if a < b { 1 } else { 0 });

                3
            }
            8 => {
                let a = decoder.next().unwrap();
                let b = decoder.next().unwrap();
                drop(decoder);

                set_argument(mode, memory, ip, base, 2, if a == b { 1 } else { 0 });

                3
            }
            9 => {
                let a = decoder.next().unwrap();
                drop(decoder);

                base += a;

                1
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

    let mut memory = vec![0; { 1024 * 1024 * 64 }];

    for (i, int) in buf
        .trim()
        .split(',')
        .map(str::trim)
        .flat_map(str::parse)
        .enumerate()
    {
        memory[i] = int;
    }

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
