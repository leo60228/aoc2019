use std::collections::{HashMap, HashSet};
use std::io::{self, prelude::*};
use rayon::prelude::*;

fn add_orbit<'a>(orbits: &mut HashMap<&'a str, HashSet<&'a str>>, a: &'a str, b: &'a str) {
    let existing: HashSet<&'a str> = orbits
        .iter()
        .filter(|(_, v)| v.contains(a))
        .map(|(&k, _)| k)
        .collect();

    for a in &existing {
        add_orbit(orbits, a, b);
    }

    orbits
        .entry(a.into())
        .or_insert_with(HashSet::new)
        .insert(b.into());
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut orbits = HashMap::new();

    for pair in input.trim().lines() {
        let mut iter = pair.split(')');
        let a = iter.next().unwrap();
        let b = iter.next().unwrap();
        add_orbit(&mut orbits, a, b);
    }

    print!("\n total: ");
    io::stdout().flush()?;
    println!("{}", orbits.into_par_iter().map(|(_, x)| x.len()).sum::<usize>());

    Ok(())
}
