
use std::fs;
use crate::aoc_errors::AocError;

pub fn solve() -> Result<u32, AocError> {
    let file = fs::read_to_string("inputs/day_02.txt")?;


    Ok(0)
}


#[cfg(test)]
mod tests {
    use crate::aoc_errors::AocError;

    #[test]
    fn test_find_numbers() -> Result<(), AocError> {
        let input = r"two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen";

        // let result = find_numbers(input.to_string())?;
        //
        // let expected = vec![29, 83, 13, 24, 42, 14, 76];
        // assert_eq!(result, expected);
        Ok(())
    }
}
