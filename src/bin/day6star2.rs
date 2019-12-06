use pathfinding::directed::dfs::dfs;
use std::collections::{HashMap, HashSet};
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

    let path = dfs(
        "YOU",
        |k| {
            orbits.get(k).into_iter().copied().chain(
                orbits
                    .iter()
                    .filter(|(_, v)| *v == k)
                    .map(|(&k, _)| k)
                    .collect::<HashSet<_>>()
                    .into_iter(),
            )
        },
        |k| *k == "SAN",
    );

    if let Some(path) = path {
        println!("path: {:?}", path);
        println!("steps: {}", path.len() - 3);
    } else {
        println!("no path");
    }

    Ok(())
}
