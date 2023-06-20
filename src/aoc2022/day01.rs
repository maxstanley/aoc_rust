use crate::{elves::iterators::top::Top, test};

use anyhow::{Context, Result};

pub fn part01(lines: &str) -> Result<usize> {
    lines
        .split("\n\n")
        .into_iter()
        .map(|v| v.split("\n").filter_map(|n| n.parse::<usize>().ok()).sum())
        .max()
        .context("no maximum value")
}

pub fn part02(lines: &str) -> Result<usize> {
    Ok(lines
        .split("\n\n")
        .into_iter()
        .map(|v| {
            v.split("\n")
                .filter_map(|n| n.parse::<usize>().ok())
                .sum::<usize>()
        })
        .top::<3>()
        .context("less that 3 elements available")?
        .iter()
        .sum::<usize>())
}

test!("2022", "day01" => 24000, 72070, 45000, 211805);
