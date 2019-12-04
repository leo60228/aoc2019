use std::io::{self, prelude::*};

pub fn exec(tape: &mut [usize]) {
    let mut cursor = 0;
    loop {
        match tape[cursor] {
            1 => {
                let a: usize = tape[cursor + 1].into();
                let b: usize = tape[cursor + 2].into();
                let c: usize = tape[cursor + 3].into();
                tape[c] = tape[a] + tape[b];
            }
            2 => {
                let a: usize = tape[cursor + 1].into();
                let b: usize = tape[cursor + 2].into();
                let c: usize = tape[cursor + 3].into();
                tape[c] = tape[a] * tape[b];
            }
            99 => break,
            _ => unimplemented!(),
        }
        cursor += 4;
    }
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;

    let mut tape: Vec<usize> = buf
        .trim()
        .split(',')
        .map(str::trim)
        .flat_map(str::parse)
        .collect();

    exec(&mut tape);

    println!(
        "{}",
        tape.iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(",")
    );

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
