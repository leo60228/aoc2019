use std::io::{self, prelude::*};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;

    let numbers = buf.chars().flat_map(|c| c.to_digit(10)).collect::<Vec<_>>();
    let layer = numbers.chunks(25 * 6).min_by_key(|chunk| chunk.iter().filter(|&&x| x == 0).count()).unwrap();
    let ones = layer.iter().filter(|&&x| x == 1).count();
    let twos = layer.iter().filter(|&&x| x == 2).count();

    println!("{}", ones * twos);

    Ok(())
}
