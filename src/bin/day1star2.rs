use std::io::{self, prelude::*};

pub fn calc_fuel(mass: usize) -> usize {
    let naive = (mass / 3).saturating_sub(2);
    if naive > 0 {
        naive + calc_fuel(naive)
    } else {
        naive
    }
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Enter masses: ");

    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;

    println!(
        "{}",
        buf.trim()
            .split_whitespace()
            .flat_map(|s| s.parse().map(calc_fuel))
            .sum::<usize>()
    );

    Ok(())
}

#[cfg(test)]
mod test {
    #[test]
    fn calc_fuel() {
        use super::calc_fuel;
        assert_eq!(calc_fuel(14), 2);
        assert_eq!(calc_fuel(1969), 966);
        assert_eq!(calc_fuel(100756), 50346);
    }
}
