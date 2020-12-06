use input::Input;
use snafu::{ResultExt, Snafu};
use snafu_cli_debug::SnafuCliDebug;
use std::collections::HashSet;

#[derive(Snafu, SnafuCliDebug)]
pub enum Error {
    /// Error loading input
    LoadingInput { source: input::Error },
    /// Error converting string to integer
    Parsing { source: std::num::ParseIntError },
}

fn parse(input: &str) -> Vec<Vec<HashSet<char>>> {
    let mut rtn = Vec::new();
    let mut current_vec = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            rtn.push(current_vec);
            current_vec = Vec::new()
        } else {
            current_vec.push(line.chars().collect::<HashSet<_>>());
        }
    }
    rtn.push(current_vec);
    rtn
}

fn part1(input: &[Vec<HashSet<char>>]) -> usize {
    input
        .iter()
        .map(|group| group.iter().fold(HashSet::new(), |acc, set| acc.union(set).copied().collect()).len())
        .sum()
}

fn part2(input: &[Vec<HashSet<char>>]) -> usize {
    input
        .iter()
        .map(|group| {
            group[1..]
                .iter()
                .fold(group[0].clone(), |acc, set| acc.intersection(set).copied().collect())
                .len()
        })
        .sum()
}

fn main() -> Result<(), Error> {
    let input = Input::open("config.toml").context(LoadingInput)?.get(6).context(LoadingInput)?;
    let input = parse(&input);
    println!("Part 1 {}", part1(&input));
    println!("Part 2 {}", part2(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let test_input = r#"abc

a
b
c

ab
ac

a
a
a
a

b"#;
        let input = parse(test_input);
        assert_eq!(part1(&input), 11);
    }

    #[test]
    fn test_part2() {
        let test_input = r#"abc

a
b
c

ab
ac

a
a
a
a

b"#;
        let input = parse(test_input);
        assert_eq!(part2(&input), 6);
    }
}
