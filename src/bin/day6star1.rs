use std::collections::HashMap;
use std::io::{self, prelude::*};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut orbits = HashMap::new();

    for pair in input.trim().lines() {
        let mut iter = pair.split(')');
        let a = iter.next().unwrap();
        let b = iter.next().unwrap();
        orbits.insert(b, a);
    }

    let mut count = 0;

    for (_, mut orbit) in &orbits {
        count += 1;
        while let Some(planet) = orbits.get(orbit) {
            count += 1;
            orbit = planet;
        }
    }

    println!("total: {}", count);

    Ok(())
}
