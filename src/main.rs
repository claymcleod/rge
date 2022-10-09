use std::env;

use anyhow::bail;
use noodles::{core::Position, fasta};

fn main() -> anyhow::Result<()> {
    let src = env::args().nth(1).expect("missing src");

    let mut reader = fasta::reader::Builder::default().build_from_path(src)?;

    // (1) Grab the sequence for both chromosome X and chromosome Y.
    let mut chr_x = None;
    let mut chr_y = None;

    for result in reader.records() {
        let record = result?;

        if record.name() == "chrX" {
            chr_x = Some(record.sequence().clone());
            println!("Hooked chromosome X!");
        } else if record.name() == "chrY" {
            chr_y = Some(record.sequence().clone());
            println!("Hooked chromosome Y!");
        }
    }

    // (2) Ensure we hooked them both. If not, return an error.
    if chr_x.is_none() {
        bail!("We didn't identify chromosome X!");
    } else if chr_y.is_none() {
        bail!("We didn't identify chromosome Y!");
    }

    println!("We hooked both chromosome X and chromosome Y!");
    println!();
    println!("----");
    println!();

    // (3) Detect start and end of PAR1

    let chr_x = chr_x.unwrap();
    let chr_y = chr_y.unwrap();

    let mut x_ptr = 1usize;
    let mut y_ptr = 1usize;

    loop {
        let position = Position::try_from(x_ptr)?;
        let base = *chr_x.get(position).unwrap() as char;

        if base != 'N' && base != 'n' {
            println!("Found first non-N base for chrX at {}", x_ptr);
            break;
        }

        x_ptr += 1;
    }

    loop {
        let position = Position::try_from(y_ptr)?;
        let base = *chr_y.get(position).unwrap() as char;

        if base != 'N' && base != 'n' {
            println!("Found first non-N base for chrY at {}", y_ptr);
            break;
        }

        y_ptr += 1;
    }

    // (3) Loop through all positions in the file
    loop {
        let x_position = Position::try_from(x_ptr)?;
        let y_position = Position::try_from(y_ptr)?;

        let x_char = chr_x.get(x_position).unwrap();
        let y_char = chr_y.get(y_position).unwrap();

        if x_char != y_char {
            println!(
                "Deviance found at position chrX: {}, chrY: {}",
                x_position, y_position
            );
            println!();

            let start = Position::try_from(x_ptr)?;
            let end = Position::try_from(x_ptr + 20)?;

            println!(
                "{:?}",
                chr_x
                    .get(start..=end)
                    .unwrap()
                    .iter()
                    .map(|x| *x as char)
                    .collect::<Vec<char>>()
            );

            let start = Position::try_from(y_ptr)?;
            let end = Position::try_from(y_ptr + 20)?;

            println!(
                "{:?}",
                chr_y
                    .get(start..=end)
                    .unwrap()
                    .iter()
                    .map(|x| *x as char)
                    .collect::<Vec<char>>()
            );
            break;
        }

        x_ptr += 1;
        y_ptr += 1;
    }

    Ok(())
}
