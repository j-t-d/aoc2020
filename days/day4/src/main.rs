use input::Input;
use snafu::{OptionExt, ResultExt, Snafu};
use snafu_cli_debug::SnafuCliDebug;
use std::collections::HashMap;

#[derive(Snafu, SnafuCliDebug)]
pub enum Error {
    /// Error loading input
    LoadingInput { source: input::Error },
    /// Error converting string to integer
    Parsing { source: std::num::ParseIntError },
    #[snafu(display("Missing required field {}", field))]
    ObtainingField { field: String },
}

struct KeyIterator<'a> {
    line: &'a str,
    pos: usize,
}

impl<'a> KeyIterator<'a> {
    pub fn new(line: &'a str) -> Self {
        Self { line, pos: 0 }
    }
}

enum Height {
    Inches(isize),
    Centimetres(isize),
}

impl<'a> Iterator for KeyIterator<'a> {
    type Item = (&'a str, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        let remainder = &self.line[self.pos..];
        if let Some(colon) = remainder.find(':') {
            let key = &remainder[..colon];
            let end = &remainder[colon + 1..];
            if let Some(space) = end.find(' ') {
                self.pos += colon + space + 2;
                let end = &end[..space];
                Some((key, end))
            } else {
                self.pos += colon + end.len() + 1;
                Some((key, end))
            }
        } else {
            None
        }
    }
}

fn parse_height(hgt: &str) -> Result<Option<Height>, Error> {
    if let Some(inches) = hgt.find("in") {
        let value = (&hgt[..inches]).parse::<isize>().context(Parsing)?;
        if &hgt[inches..] == "in" {
            Ok(Some(Height::Inches(value)))
        } else {
            Ok(None)
        }
    } else if let Some(cm) = hgt.find("cm") {
        let value = (&hgt[..cm]).parse::<isize>().context(Parsing)?;
        if &hgt[cm..] == "cm" {
            Ok(Some(Height::Centimetres(value)))
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}

fn verify_hcl(hcl: &str) -> bool {
    hcl.chars().nth(0) == Some('#') && hcl.len() == 7 && (&hcl[1..]).chars().filter(|c| c.is_alphanumeric()).count() == 6
}

fn verify_ecl(ecl: &str) -> bool {
    let valid = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    valid.iter().filter(|e| ecl == **e).count() == 1
}

fn verify_pid(pid: &str) -> bool {
    pid.chars().filter(|c| c.is_numeric()).count() == 9
}

fn verify_hgt(hgt: &Height) -> bool {
    match *hgt {
        Height::Centimetres(val) => val >= 150 && val <= 193,
        Height::Inches(val) => val >= 59 && val <= 76,
    }
}

fn verify_field_values(passport: &HashMap<&str, &str>) -> Result<bool, Error> {
    let byr = passport
        .get("byr")
        .context(ObtainingField { field: "byr" })?
        .parse::<isize>()
        .context(Parsing)?;
    let iyr = passport
        .get("iyr")
        .context(ObtainingField { field: "iyr" })?
        .parse::<isize>()
        .context(Parsing)?;
    let eyr = passport
        .get("eyr")
        .context(ObtainingField { field: "eyr" })?
        .parse::<isize>()
        .context(Parsing)?;

    let hgt = parse_height(passport.get("hgt").context(ObtainingField { field: "hgt" })?)?.context(ObtainingField { field: "hgt" })?;
    let hcl = *passport.get("hcl").context(ObtainingField { field: "hcl" })?;
    let ecl = *passport.get("ecl").context(ObtainingField { field: "ecl" })?;
    let pid = *passport.get("pid").context(ObtainingField { field: "pid" })?;

    let byr = byr >= 1920 && byr <= 2002;
    let iyr = iyr >= 2010 && iyr <= 2020;
    let eyr = eyr >= 2020 && eyr <= 2030;
    let hgt = verify_hgt(&hgt);
    let hcl = verify_hcl(hcl);
    let ecl = verify_ecl(ecl);
    let pid = verify_pid(pid);

    Ok(byr && iyr && eyr && hgt && hcl && ecl && pid)
}

fn verify_fields(passport: &HashMap<&str, &str>) -> bool {
    let required_keys = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    required_keys.iter().filter(|key| passport.contains_key(**key)).count() == required_keys.len()
}

fn part1(input: &Vec<HashMap<&str, &str>>) -> usize {
    input.iter().filter(|pass| verify_fields(*pass)).count()
}

fn part2(input: &Vec<HashMap<&str, &str>>) -> usize {
    input
        .iter()
        .filter(|pass| match verify_field_values(*pass) {
            Ok(valid) => valid,
            Err(_e) => false,
        })
        .count()
}

fn parse(input: &str) -> Vec<HashMap<&str, &str>> {
    let mut rtn = Vec::new();
    let mut current = HashMap::new();
    for line in input.lines() {
        if line.is_empty() {
            rtn.push(current);
            current = HashMap::new();
        } else {
            KeyIterator::new(line).for_each(|(key, value)| {
                current.insert(key, value);
            });
        }
    }
    rtn.push(current);
    rtn
}

fn main() -> Result<(), Error> {
    let input = Input::open("config.toml").context(LoadingInput)?.get(4).context(LoadingInput)?;
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
        let test_input = r#"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"#;
        let input = parse(test_input);
        assert_eq!(input.len(), 4);
        assert_eq!(part1(&input), 2);
    }

    #[test]
    fn test_part2_all_bad() {
        let test_input = r#"eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007"#;
        let input = parse(test_input);
        assert_eq!(part2(&input), 0);
    }

    #[test]
    fn test_part2_all_good() {
        let test_input = r#"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"#;
        let input = parse(test_input);
        assert_eq!(part2(&input), 4);
    }
}
