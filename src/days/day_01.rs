use std::fs;
use std::str::Chars;
use crate::aoc_errors::AocError;

pub fn solve() -> Result<u32, AocError> {
    let file = fs::read_to_string("inputs/day_01.txt")?;
    let numbers = find_numbers(file)?;
    let sum = numbers.iter().sum();
    Ok(sum)
}

fn find_numbers(file: String) -> Result<Vec<u32>, AocError> {
    let mut numbers: Vec<u32> = Vec::new();
    for line in file.lines() {
        let first = find_alpha_number(line.chars().clone());
        let last = find_alpha_number(line.chars().clone().rev().collect::<String>().chars());
        let number = first * 10 + last;
        numbers.push(number)
    }
    Ok(numbers)
}

fn find_alpha_number(chars: Chars) -> u32 {
    let numbers_as_str: Vec<(&str, u32)> = vec![
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    let mut substring = String::new();
    for char in chars {
        if char.is_numeric() { return char.to_string().parse().unwrap(); };
        substring.push(char);
        let number = numbers_as_str
            .iter()
            .find(|x| substring.contains(x.0) || substring.chars().rev().collect::<String>().contains(x.0));
        if let Some((_, x)) = number { return *x; }
    }

    panic!("xD")
}


#[cfg(test)]
mod tests {
    use std::io::Error;
    use crate::aoc_errors::AocError;
    use crate::days::day_01::find_numbers;

    #[test]
    fn test_find_numbers() -> Result<(), AocError> {
        let input = r"two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen";

        let result = find_numbers(input.to_string())?;

        let expected = vec![29, 83, 13, 24, 42, 14, 76];
        assert_eq!(result, expected);
        Ok(())
    }
}
