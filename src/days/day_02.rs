use std::fs;
use crate::aoc_errors::AocError;

pub fn solve() -> Result<u32, AocError> {
    let file = fs::read_to_string("inputs/day_02.txt")?;
    let sum = solve_inner(file)?;
    Ok(sum)
}

fn solve_inner(file: String) -> Result<u32, AocError> {
    let mut games: Vec<Game> = Vec::new();
    for line in file.lines() {
        let game = Game::from_line(line);
        games.push(game)
    }

    let sum = games
        .iter()
        .map(|x| {
            let max_red = x.rounds.iter().map(|r| r.red).max().unwrap();
            let max_blue = x.rounds.iter().map(|r| r.blue).max().unwrap();
            let max_green = x.rounds.iter().map(|r| r.green).max().unwrap();
            max_red * max_blue * max_green
        })
        .sum();

    Ok(sum)
}

struct Game {
    id: u32,
    rounds: Vec<Round>,
}

impl Game {
    fn from_line(line: &str) -> Game {
        //Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        let id = line.split(':')
            .next().unwrap()
            .split(' ')
            .last().unwrap()
            .parse().unwrap();

        let rounds = line.split(':')
            .last()
            .unwrap().split(';')
            .map(|x| Round::from_line(x))
            .collect::<Vec<Round>>();

        Game { id, rounds }
    }
}

struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

impl Round {
    fn from_line(line: &str) -> Round {
        //6 red, 1 blue, 3 green
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        line.split(',')
            .for_each(|x| {
                let pair: Vec<&str> = x.split_whitespace().collect();
                let value: u32 = pair.first().unwrap().parse().unwrap();
                match *pair.last().unwrap() {
                    "red" => red = value,
                    "green" => green = value,
                    "blue" => blue = value,
                    _ => panic!()
                };
            });

        Round { red, green, blue }
    }
}


#[cfg(test)]
mod tests {
    use crate::aoc_errors::AocError;
    use crate::days::day_02::solve_inner;

    #[test]
    fn test_find_numbers() -> Result<(), AocError> {
        let input = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let result = solve_inner(input.to_string())?;

        let expected: u32 = 2286;
        assert_eq!(result, expected);
        Ok(())
    }
}
