use input::Input;
use snafu::{ResultExt, Snafu};
use snafu_cli_debug::SnafuCliDebug;
use std::cmp::{Ord, Ordering};
use std::ops::Range;

#[derive(Snafu, SnafuCliDebug)]
pub enum Error {
    /// Error loading input
    LoadingInput { source: input::Error },
    /// Error converting string to integer
    Parsing { source: std::num::ParseIntError },
}

#[derive(Debug, Eq, PartialEq)]
struct Seat {
    row: usize,
    col: usize,
    id: usize,
}

impl Ord for Seat {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for Seat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse(input: &str) -> Vec<&str> {
    input.lines().collect::<Vec<_>>()
}

fn half(range: &Range<usize>) -> usize {
    range.start + (range.end - range.start) / 2
}

fn find_seat(seat: &str, rows: Range<usize>, columns: Range<usize>) -> Seat {
    let (rows, columns) = match seat.chars().next() {
        Some('B') => (half(&rows)..rows.end, columns),
        Some('F') => (rows.start..half(&rows), columns),
        Some('L') => (rows, columns.start..half(&columns)),
        Some('R') => (rows, half(&columns)..columns.end),
        None => {
            assert_eq!(rows.len(), 1);
            assert_eq!(columns.len(), 1);
            let row = rows.start;
            let col = columns.start;
            return Seat { row, col, id: row * 8 + col };
        }
        _ => unreachable!(),
    };
    find_seat(&seat[1..], rows, columns)
}

fn part1(input: &[&str]) -> usize {
    input.iter().map(|seat| find_seat(seat, 0..128, 0..8)).max().map(|seat| seat.id).unwrap_or(0)
}

fn part2(input: &[&str]) -> usize {
    let mut ids = input.iter().map(|seat| find_seat(seat, 0..128, 0..8)).collect::<Vec<_>>();
    ids.sort();
    let mut prev = ids[0].id;
    for seat in &ids[1..] {
        if seat.id - 1 != prev {
            return seat.id - 1;
        }
        prev = seat.id;
    }
    0
}

fn main() -> Result<(), Error> {
    let input = Input::open("config.toml").context(LoadingInput)?.get(5).context(LoadingInput)?;
    let input = parse(&input);
    println!("Part 1 {}", part1(&input));
    println!("Part 2 {}", part2(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let test_input = r#"FBFBBFFRLR
BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL"#;
        let input = parse(test_input);
        assert_eq!(find_seat(input[0], 0..128, 0..8), Seat { row: 44, col: 5, id: 357 });
        assert_eq!(find_seat(input[1], 0..128, 0..8), Seat { row: 70, col: 7, id: 567 });
        assert_eq!(find_seat(input[2], 0..128, 0..8), Seat { row: 14, col: 7, id: 119 });
        assert_eq!(find_seat(input[3], 0..128, 0..8), Seat { row: 102, col: 4, id: 820 });
    }

    #[test]
    fn test_part1() {
        let test_input = r#"FBFBBFFRLR
BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL"#;
        let input = parse(test_input);
        assert_eq!(part1(&input), 820)
    }
}
