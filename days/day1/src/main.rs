use input::Input;
use snafu::{ResultExt, Snafu};
use snafu_cli_debug::SnafuCliDebug;

#[derive(Snafu, SnafuCliDebug)]
pub enum Error {
    /// Error loading input
    LoadingInput { source: input::Error },
    /// Error converting string to integer
    Parsing { source: std::num::ParseIntError },
}

fn parse(input: &str) -> Result<Vec<isize>, Error> {
    Ok(input.lines().map(|l| l.parse::<isize>().context(Parsing)).collect::<Result<Vec<_>, Error>>()?)
}

fn part1(input: &Vec<isize>) -> Result<Option<isize>, Error> {
    for (index1, value1) in input.iter().enumerate() {
        for (index2, value2) in input.iter().enumerate() {
            if index1 != index2 {
                if value1 + value2 == 2020 {
                    return Ok(Some(value1 * value2));
                }
            }
        }
    }
    Ok(None)
}

fn part2(input: &Vec<isize>) -> Result<Option<isize>, Error> {
    for (index1, value1) in input.iter().enumerate() {
        for (index2, value2) in input.iter().enumerate() {
            for (index3, value3) in input.iter().enumerate() {
                if index1 != index2 && index1 != index3 && index2 != index3 {
                    if value1 + value2 + value3 == 2020 {
                        return Ok(Some(value1 * value2 * value3));
                    }
                }
            }
        }
    }
    Ok(None)
}

fn main() -> Result<(), Error> {
    let input = Input::open("config.toml").context(LoadingInput)?.get(1).context(LoadingInput)?;
    let input = parse(&input)?;
    println!("Part 1 {}", part1(&input)?.unwrap());
    println!("Part 2 {}", part2(&input)?.unwrap());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let test_input = r#"1721
979
366
299
675
1456"#;
        let input = parse(test_input).expect("parse");
        assert_eq!(part1(&input).unwrap(), Some(514579));
    }

    #[test]
    fn test_part2() {
        let test_input = r#"1721
979
366
299
675
1456"#;
        let input = parse(test_input).expect("parse");
        assert_eq!(part2(&input).unwrap(), Some(241861950));
    }
}
