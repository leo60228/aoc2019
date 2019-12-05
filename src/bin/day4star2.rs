use arrayvec::ArrayVec;
use std::convert::TryInto;
use std::io;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Password([u8; 6]);

impl From<Password> for usize {
    fn from(c: Password) -> Self {
        c.0.iter()
            .rev()
            .enumerate()
            .map(|(i, d)| usize::from(*d) * 10usize.pow(i.try_into().unwrap()))
            .sum()
    }
}

impl From<usize> for Password {
    fn from(c: usize) -> Self {
        let vec: ArrayVec<[_; 6]> = (0..6)
            .rev()
            .map(|i| ((c / 10usize.pow(i)) % 10) as u8)
            .collect();
        Self(vec.into_inner().unwrap())
    }
}

pub fn valid_password(pw: &Password) -> bool {
    let mut highest = 0;
    let mut last = None;
    let mut multi = 0;
    let mut found_two = false;

    for &digit in &pw.0 {
        highest = highest.max(digit);

        if digit < highest {
            return false;
        }

        if last == Some(digit) {
            found_two = multi == 1;
        } else {
            multi = 0;
        }

        multi += 1;

        last = Some(digit);
    }

    found_two
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    let x: usize = buf.trim().parse().unwrap();

    buf.clear();
    io::stdin().read_line(&mut buf)?;
    let y: usize = buf.trim().parse().unwrap();

    println!(
        "{}",
        (x..=y).map(Password::from).filter(valid_password).count()
    );

    Ok(())
}

#[cfg(test)]
mod test {
    #[test]
    fn valid_password() {
        use super::valid_password;
        assert!(valid_password(&112233usize.into()));
        assert!(!valid_password(&123444usize.into()));
        assert!(valid_password(&111122usize.into()));
        assert!(valid_password(&111223usize.into()));
        assert!(valid_password(&122344usize.into()));
        assert!(valid_password(&122345usize.into()));
        assert!(valid_password(&123455usize.into()));
        assert!(valid_password(&112345usize.into()));
        assert!(valid_password(&111233usize.into()));
    }
}
