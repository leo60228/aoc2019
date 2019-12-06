use rayon::prelude::*;
use std::collections::HashMap;
use std::io::{self, prelude::*};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let orbits: HashMap<_, _> = input
        .trim()
        .lines()
        .map(|pair| {
            let mut iter = pair.split(')');
            let a = iter.next().unwrap();
            let b = iter.next().unwrap();
            (b, a)
        })
        .collect();

    let count: usize = orbits
        .par_iter()
        .map(|(_, mut orbit)| {
            let mut count = 1;
            while let Some(planet) = orbits.get(orbit) {
                count += 1;
                orbit = planet;
            }
            count
        })
        .sum();

    println!("total: {}", count);

    Ok(())
}
