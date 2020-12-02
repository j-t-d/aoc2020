use input::Input;
use regex::Regex;
use snafu::{OptionExt, ResultExt, Snafu};
use snafu_cli_debug::SnafuCliDebug;
use std::ops::RangeInclusive;

#[derive(Snafu, SnafuCliDebug)]
pub enum Error {
    /// Error loading input
    LoadingInput { source: input::Error },
    /// Error during regex operation
    RegexOp { source: regex::Error },
    /// Failed to parse input
    Parsing { source: std::num::ParseIntError },
    /// Missing single letter on password
    Letter,
    /// Regex didn't match
    RegexMatch,
}

#[derive(Debug)]
struct Password {
    frequency: RangeInclusive<usize>,
    letter: char,
    password: String,
}

fn part1(input: &Vec<Password>) -> usize {
    input
        .iter()
        .filter_map(|pass| {
            let count = pass
                .password
                .chars()
                .filter_map(|char| if char == pass.letter { Some(true) } else { None })
                .count();
            if pass.frequency.contains(&count) {
                Some(true)
            } else {
                None
            }
        })
        .count()
}

fn part2(input: &Vec<Password>) -> usize {
    input
        .iter()
        .filter_map(|pass| {
            let start = pass.password.chars().nth(*pass.frequency.start() - 1).unwrap();
            let end = pass.password.chars().nth(*pass.frequency.end() - 1).unwrap();
            if (start == pass.letter || end == pass.letter) && start != end {
                Some(true)
            } else {
                None
            }
        })
        .count()
}

fn parse(input: &str) -> Result<Vec<Password>, Error> {
    let regex = Regex::new(r#"^(\d+)-(\d+) ([a-z]): ([a-z]+)"#).context(RegexOp)?;
    input
        .lines()
        .map(|l| {
            let captures = regex.captures(l).context(RegexMatch)?;
            Ok(Password {
                frequency: captures[1].parse::<usize>().context(Parsing)?..=captures[2].parse::<usize>().context(Parsing)?,
                letter: captures[3].chars().nth(0).context(Letter)?,
                password: (&captures[4]).to_string(),
            })
        })
        .collect::<Result<Vec<_>, Error>>()
}

fn main() -> Result<(), Error> {
    let input = Input::open("config.toml").context(LoadingInput)?.get(2).context(LoadingInput)?;
    let input = parse(&input)?;
    println!("Part 1 {}", part1(&input));
    println!("Part 2 {}", part2(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let test_input = r#"1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc"#;
        let input = parse(test_input).expect("parse");
        assert_eq!(part1(&input), 2);
    }

    #[test]
    fn test_part2() {
        let test_input = r#"1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc"#;
        let input = parse(test_input).expect("parse");
        assert_eq!(part2(&input), 1);
    }
}
