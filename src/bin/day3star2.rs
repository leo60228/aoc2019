use std::collections::HashMap;
use std::io;

pub fn points(desc: &str) -> Result<HashMap<(isize, isize), usize>, Box<dyn std::error::Error>> {
    let mut map = HashMap::new();
    let mut loc = (0, 0);
    let mut i = 0;

    for instr in desc.split(',') {
        let op = &instr[0..1];
        let num = &instr[1..];
        let num: usize = num.parse()?;
        for _ in 0..num {
            i += 1;
            match op {
                "R" => loc.0 += 1,
                "L" => loc.0 -= 1,
                "U" => loc.1 += 1,
                "D" => loc.1 -= 1,
                _ => unimplemented!(),
            }
            map.insert(loc, i);
        }
    }

    Ok(map)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    let a = points(buf.trim())?;

    buf.clear();
    io::stdin().read_line(&mut buf)?;
    let b = points(buf.trim())?;

    println!(
        "{:?}",
        a.into_iter()
            .filter_map(|(p, i)| b.get(&p).map(|i2| (p, i + i2)))
            .min_by_key(|x| x.1)
    );

    Ok(())
}
