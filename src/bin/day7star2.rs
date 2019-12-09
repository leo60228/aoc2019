use permutohedron::Heap;
use std::convert::{TryFrom, TryInto};
use std::io::{self, prelude::*};
use std::iter;

pub fn decode<'a>(
    mode: isize,
    mut ip: usize,
    memory: &'a [isize],
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
            _ => unimplemented!(),
        })
    })
}

pub fn exec(
    memory: &mut [isize],
    mut ip: usize,
    mut input: isize,
    input2: isize,
    skip_input: bool,
) -> Option<(usize, isize)> {
    if skip_input {
        input = input2;
    }
    loop {
        let instr = memory[ip];
        let opcode = instr % 100;
        let mode = instr / 100;
        let mut decoder = decode(mode, ip, &memory);
        let arity = match opcode {
            1 => {
                let a = decoder.next().unwrap();
                let b = decoder.next().unwrap();
                let c = memory[ip + 3];
                drop(decoder);

                memory[usize::try_from(c).unwrap()] = a + b;
                3
            }
            2 => {
                let a = decoder.next().unwrap();
                let b = decoder.next().unwrap();
                let c = memory[ip + 3];
                drop(decoder);

                memory[usize::try_from(c).unwrap()] = a * b;
                3
            }
            3 => {
                let a = memory[ip + 1];
                drop(decoder);

                memory[usize::try_from(a).unwrap()] = input;
                input = input2;
                1
            }
            4 => {
                let a = decoder.next().unwrap();
                drop(decoder);

                ip += 2;

                return Some((ip, a));
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
                let c = memory[ip + 3];
                drop(decoder);

                memory[usize::try_from(c).unwrap()] = if a < b { 1 } else { 0 };

                3
            }
            8 => {
                let a = decoder.next().unwrap();
                let b = decoder.next().unwrap();
                let c = memory[ip + 3];
                drop(decoder);

                memory[usize::try_from(c).unwrap()] = if a == b { 1 } else { 0 };

                3
            }
            99 => return None,
            unimp => unimplemented!("unimplemented opcode {}", unimp),
        };
        ip += (arity + 1) as usize;
    }
}

fn highest(arr: &[isize], program: &Vec<isize>) -> isize {
    let mut a_mem = program.clone();
    let mut a_ip = 0;
    let mut b_mem = program.clone();
    let mut b_ip = 0;
    let mut c_mem = program.clone();
    let mut c_ip = 0;
    let mut d_mem = program.clone();
    let mut d_ip = 0;
    let mut e_mem = program.clone();
    let mut e_ip = 0;
    let mut output = 0;
    let mut e_out = 0;
    let mut looped = false;
    loop {
        match exec(&mut a_mem, a_ip, arr[0], output, looped) {
            Some((ip, o)) => {
                a_ip = ip;
                output = o;
            }
            None => break e_out,
        }
        match exec(&mut b_mem, b_ip, arr[1], output, looped) {
            Some((ip, o)) => {
                b_ip = ip;
                output = o;
            }
            None => break e_out,
        }
        match exec(&mut c_mem, c_ip, arr[2], output, looped) {
            Some((ip, o)) => {
                c_ip = ip;
                output = o;
            }
            None => break e_out,
        }
        match exec(&mut d_mem, d_ip, arr[3], output, looped) {
            Some((ip, o)) => {
                d_ip = ip;
                output = o;
            }
            None => break e_out,
        }
        match exec(&mut e_mem, e_ip, arr[4], output, looped) {
            Some((ip, o)) => {
                e_ip = ip;
                output = o;
                e_out = o;
            }
            None => break e_out,
        }
        looped = true;
    }
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;

    let program: Vec<isize> = buf
        .trim()
        .split(',')
        .map(str::trim)
        .flat_map(str::parse)
        .collect();

    let mut possibilities = [5, 6, 7, 8, 9];
    let heap = Heap::new(&mut possibilities);

    let phase = heap.map(|arr| highest(&arr, &program)).max();

    println!("{:?}", phase);

    Ok(())
}

#[cfg(test)]
mod test {
    #[test]
    fn highest() {
        let program = vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];
        assert_eq!(super::highest(&[9, 8, 7, 6, 5], &program), 139629729);
    }
}
