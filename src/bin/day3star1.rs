use std::collections::HashSet;
use std::io;

pub fn points(desc: &str) -> Result<HashSet<(isize, isize)>, Box<dyn std::error::Error>> {
    let mut set = HashSet::new();
    let mut loc = (0, 0);

    for instr in desc.split(',') {
        let op = &instr[0..1];
        let num = &instr[1..];
        let num: usize = num.parse()?;
        for _ in 0..num {
            match op {
                "R" => loc.0 += 1,
                "L" => loc.0 -= 1,
                "U" => loc.1 += 1,
                "D" => loc.1 -= 1,
                _ => unimplemented!(),
            }
            set.insert(loc);
        }
    }

    Ok(set)
}

pub fn manhattan(a: (isize, isize), b: (isize, isize)) -> isize {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    let a = points(buf.trim())?;

    buf.clear();
    io::stdin().read_line(&mut buf)?;
    let b = points(buf.trim())?;

    for point in a.intersection(&b) {
        println!("{:?} {}", point, manhattan(*point, (0, 0)));
    }

    println!(
        "{:?}",
        a.intersection(&b)
            .map(|&p| (p, manhattan(p, (0, 0))))
            .min_by_key(|x| x.1)
    );

    Ok(())
}
