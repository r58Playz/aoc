use std::{collections::HashMap, env, fs};

use anyhow::{Context, Result};

pub fn main() -> Result<()> {
    let input = env::args().nth(1).context("no input")?;
    let input = fs::read_to_string(input).context("failed to read input")?;

    let input_lines = input.lines().count();

    let mut left: Vec<u64> = Vec::with_capacity(input_lines);
    let mut right: Vec<u64> = Vec::with_capacity(input_lines);

    for line in input.lines() {
        let mut split = line.split("   ");
        left.push(
            split
                .next()
                .context("failed to get left input")?
                .parse()
                .context("failed to parse input")?,
        );
        right.push(
            split
                .next()
                .context("failed to get left input")?
                .parse()
                .context("failed to parse input")?,
        );
    }

    left.sort();
    right.sort();

	println!("{:?}", left);

    let mut dist = 0usize;
    for (litem, ritem) in left.iter().zip(right.iter()) {
        dist += (*litem as i128 - *ritem as i128).unsigned_abs() as usize;
    }

    println!("dist {:?}", dist);

    let mut count: HashMap<u64, u64> = HashMap::new();

    for itm in right {
        if let Some(cnt) = count.get_mut(&itm) {
            *cnt += 1;
        } else {
            count.insert(itm, 1);
        }
    }

    let mut similarity = 0u64;
    for itm in left {
        similarity += itm * count.get(&itm).unwrap_or(&0);
    }

    println!("similarity {:?}", similarity);

    Ok(())
}
