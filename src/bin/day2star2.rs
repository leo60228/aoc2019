use std::io::{self, prelude::*};

pub fn exec(memory: &mut [usize]) {
    let mut ip = 0;
    loop {
        let arity = match memory[ip] {
            1 => {
                let a: usize = memory[ip + 1].into();
                let b: usize = memory[ip + 2].into();
                let c: usize = memory[ip + 3].into();
                memory[c] = memory[a] + memory[b];
                3
            }
            2 => {
                let a: usize = memory[ip + 1].into();
                let b: usize = memory[ip + 2].into();
                let c: usize = memory[ip + 3].into();
                memory[c] = memory[a] * memory[b];
                3
            }
            99 => break,
            _ => unimplemented!(),
        };
        ip += arity + 1;
    }
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;

    let program: Vec<usize> = buf
        .trim()
        .split(',')
        .map(str::trim)
        .flat_map(str::parse)
        .collect();

    for noun in 0..100 {
        for verb in 0..100 {
            let mut memory = program.clone();
            memory[1] = noun;
            memory[2] = verb;
            exec(&mut memory);
            if memory[0] == 19690720 {
                dbg!(noun, verb);
                break;
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {
    #[test]
    fn exec() {
        use super::exec;
        fn check(inp: &[usize], out: &[usize]) {
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
