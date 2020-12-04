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

enum Coordinate {
    Open,
    Tree,
}

fn parse(input: &str) -> Vec<Vec<Coordinate>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '#' => Coordinate::Tree,
                    '.' => Coordinate::Open,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn calc_tree(input: &Vec<Vec<Coordinate>>, x_step: usize, y_step: usize) -> usize {
    let mut x = 0;
    input
        .iter()
        .step_by(y_step)
        .filter(|line| {
            let rtn = match line[x] {
                Coordinate::Tree => true,
                Coordinate::Open => false,
            };
            x += x_step;
            if x >= line.len() {
                x = x - line.len();
            }
            rtn
        })
        .count()
}

fn part1(input: &Vec<Vec<Coordinate>>) -> usize {
    calc_tree(input, 3, 1)
}

fn part2(input: &Vec<Vec<Coordinate>>) -> usize {
    calc_tree(input, 1, 1) * calc_tree(input, 3, 1) * calc_tree(input, 5, 1) * calc_tree(input, 7, 1) * calc_tree(input, 1, 2)
}

fn main() -> Result<(), Error> {
    let input = Input::open("config.toml").context(LoadingInput)?.get(3).context(LoadingInput)?;
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
        let test_input = r#"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"#;
        let input = parse(test_input);
        assert_eq!(part1(&input), 7);
    }

    #[test]
    fn test_part2() {
        let test_input = r#"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"#;
        let input = parse(test_input);
        assert_eq!(part2(&input), 336);
    }
}
