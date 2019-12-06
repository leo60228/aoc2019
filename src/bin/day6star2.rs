use std::collections::{HashMap, HashSet};
use std::io::{self, prelude::*};

fn parents<'a>(orbits: &HashMap<&'a str, &'a str>, mut orbit: &'a str) -> HashSet<&'a str> {
    let mut parents = HashSet::new();
    parents.insert(orbit);

    while let Some(planet) = orbits.get(orbit) {
        parents.insert(planet);
        orbit = planet;
    }

    parents
}

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

    let you = parents(&orbits, "YOU");
    let san = parents(&orbits, "SAN");
    let intersection = you.intersection(&san).count();
    let steps = you.len() + san.len() - (2 * intersection) - 2;

    println!("steps: {:?}", steps);

    Ok(())
}
