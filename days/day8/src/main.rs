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

#[derive(Debug, Clone)]
enum Op {
    Nop,
    Jmp,
    Acc,
    Unsupported,
}

#[derive(Debug, Clone)]
struct Instruction {
    op: Op,
    value: isize,
    visited: bool,
}

fn process(mut input: Vec<Instruction>) -> (bool, isize) {
    let mut ip = 0;
    let mut acc = 0;
    loop {
        let instruction = &mut input[ip as usize];
        if instruction.visited {
            break (false, acc);
        }
        match instruction.op {
            Op::Acc => {
                acc += instruction.value;
                ip += 1;
            }
            Op::Nop => ip += 1,
            Op::Jmp => ip += instruction.value,
            _ => unimplemented!(),
        }
        instruction.visited = true;
        if ip as usize == input.len() {
            break (true, acc);
        }
    }
}

fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|l| {
            let mut instr = l.split(' ');
            let op = match instr.next().unwrap() {
                "jmp" => Op::Jmp,
                "nop" => Op::Nop,
                "acc" => Op::Acc,
                _ => Op::Unsupported,
            };
            let value = instr.next().unwrap().parse::<isize>().unwrap();
            Instruction { op, value, visited: false }
        })
        .collect()
}

fn part1(input: &[Instruction]) -> isize {
    let input = input.to_vec();
    process(input).1
}

fn part2(input: &[Instruction]) -> isize {
    let mut iter = input.iter().enumerate();
    loop {
        if let Some((index, instr)) = iter.next() {
            match instr.op {
                Op::Jmp => {
                    let mut test_input = input.to_vec();
                    test_input[index].op = Op::Nop;
                    let (completed, acc) = process(test_input);
                    if completed {
                        break acc;
                    }
                }
                Op::Nop => {
                    let mut test_input = input.to_vec();
                    test_input[index].op = Op::Jmp;
                    let (completed, acc) = process(test_input);
                    if completed {
                        break acc;
                    }
                }
                _ => {}
            }
        } else {
            break 0;
        }
    }
}

fn main() -> Result<(), Error> {
    let input = Input::open("config.toml").context(LoadingInput)?.get(8).context(LoadingInput)?;
    let input = parse(&input);
    println!("Part 1 {}", part1(&input));
    eprintln!("Part 2 {}", part2(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let test_input = r#"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"#;
        let input = parse(test_input);
        let value = part1(&input);
        assert_eq!(value, 5);
    }

    #[test]
    fn test_part2() {
        let test_input = r#"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"#;
        let input = parse(test_input);
        let value = part2(&input);
        assert_eq!(value, 8);
    }
}
