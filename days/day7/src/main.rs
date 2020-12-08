use input::Input;
use petgraph::graphmap::DiGraphMap;
use petgraph::Direction;
use regex::Regex;
use snafu::{OptionExt, ResultExt, Snafu};
use snafu_cli_debug::SnafuCliDebug;
use std::collections::{HashMap, HashSet};

#[derive(Snafu, SnafuCliDebug)]
pub enum Error {
    /// Error loading input
    LoadingInput { source: input::Error },
    /// Error converting string to integer
    Parsing { source: std::num::ParseIntError },
    /// Regex Error
    CompilingRegex { source: regex::Error },
    /// MissingName
    MissingName,
}

struct Bag<'a> {
    name: &'a str,
    bags: HashMap<&'a str, usize>,
}

fn parse_line(mut line: &str) -> Result<Bag, Error> {
    let first = Regex::new(r#"^(.+)(?: bags contain)"#).context(CompilingRegex)?;
    let second = Regex::new(r#"^\s?(\d+) (.*) bag"#).context(CompilingRegex)?;
    let mut name = None;

    if let Some(captures) = first.captures(line) {
        name = Some(captures.get(1).context(MissingName)?.as_str());
        line = &line[captures[0].len()..];
    }
    let name = name.context(MissingName)?;
    let bags = line
        .split(',')
        .filter_map(|s| {
            if let Some(captures) = second.captures(s) {
                let bag_name = captures.get(2).expect("No capture").as_str();
                let count = (&captures[1]).parse::<usize>().expect("No paarse");
                Some((bag_name, count))
            } else {
                None
            }
        })
        .collect::<HashMap<_, _>>();
    Ok(Bag { name, bags })
}

fn parse(input: &str) -> Result<DiGraphMap<&str, usize>, Error> {
    let mut graph = DiGraphMap::new();
    let bags = input.lines().map(|l| parse_line(l)).collect::<Result<Vec<_>, Error>>()?;
    bags.iter().for_each(|bag| {
        graph.add_node(bag.name);
        for (name, count) in bag.bags.iter() {
            graph.add_node(*name);
            graph.add_edge(bag.name, *name, *count);
        }
    });
    Ok(graph)
}

fn part2_process<'a>(input: &DiGraphMap<&'a str, usize>, bag: &'a str) -> usize {
    let mut count = 0;
    input.neighbors_directed(bag, Direction::Outgoing).for_each(|s| {
        let edge = *(input.edge_weight(bag, s).unwrap());
        let edge_bags = part2_process(input, s);
        count += edge + edge * edge_bags;
    });
    count
}

fn part1_process<'a>(input: &DiGraphMap<&'a str, usize>, bag: &'a str, set: &mut HashSet<&'a str>) {
    input.neighbors_directed(bag, Direction::Incoming).for_each(|s| {
        set.insert(s);
        part1_process(input, s, set);
    })
}

fn part2(input: &DiGraphMap<&str, usize>) -> usize {
    part2_process(input, "shiny gold")
}

fn part1(input: &DiGraphMap<&str, usize>) -> usize {
    let mut set = HashSet::new();
    part1_process(input, "shiny gold", &mut set);
    set.len()
}

fn main() -> Result<(), Error> {
    let input = Input::open("config.toml").context(LoadingInput)?.get(7).context(LoadingInput)?;
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
        let test_input = r#"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."#;
        let input = parse(test_input).expect("parse");
        assert_eq!(part1(&input), 4);
    }

    #[test]
    fn test_part2a() {
        let test_input = r#"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."#;
        let input = parse(test_input).expect("parse");
        assert_eq!(part2(&input), 32);
    }

    #[test]
    fn test_part2b() {
        let test_input = r#"shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags."#;
        let input = parse(test_input).expect("parse");
        assert_eq!(part2(&input), 126);
    }
}
