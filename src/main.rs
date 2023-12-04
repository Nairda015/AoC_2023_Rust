mod days;
mod aoc_errors;

use crate::aoc_errors::AocError;
use days::day_02;

fn main() -> Result<(), AocError> {
    let result = day_02::solve()?;
    println!("{}", result);
    Ok(())
}