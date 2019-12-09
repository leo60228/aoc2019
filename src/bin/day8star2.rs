use std::io::{self, prelude::*};

const WIDTH: usize = 25;
const HEIGHT: usize = 6;
const SIZE: usize = WIDTH * HEIGHT;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;

    let mut image = [2; SIZE];
    let numbers = buf.chars().flat_map(|c| c.to_digit(10)).collect::<Vec<_>>();

    for layer in numbers.chunks(SIZE).rev() {
        for (i, &pixel) in layer.iter().enumerate() {
            image[i] = match (pixel, image[i]) {
                (2, img) => img,
                (pix, _) => pix,
            };
        }
    }

    for row in image.chunks(WIDTH) {
        for pixel in row {
            print!("{}", match pixel {
                0 => "█",
                1 => "░",
                2 => " ",
                _ => unreachable!(),
            });
        }
        println!("");
    }

    Ok(())
}
