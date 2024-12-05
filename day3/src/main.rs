use std::fmt::Debug;

use env_logger;
use log::{self, info};
use regex::{Match, Regex};
type Num = u32;
struct MultOps {
    x: Num,
    y: Num,
}
impl MultOps {
    pub fn new(x: Num, y: Num) -> MultOps {
        MultOps { x, y }
    }
    pub fn calc(&self) -> Num {
        self.x * self.y
    }
}
fn part1(input: &str) {
    // capture the digits in a regex
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let ops: Vec<MultOps> = re
        .captures_iter(input)
        .filter_map(|reg_match| {
            // note this will silently fail if parsing fails
            let (x, y) = (
                reg_match.get(1)?.as_str().parse::<Num>().ok()?,
                reg_match.get(2)?.as_str().parse::<Num>().ok()?,
            );
            info!(target:"Operand Filter","{}x{}",x,y);
            Some(MultOps::new(x, y))
        })
        .collect();
    println!(
        "Total: {}",
        ops.iter().fold(0, |total, op| total + op.calc())
    );
}
// part 2 stuff
#[derive(Debug, PartialEq)]
enum Modifier {
    None,
    Do,
    Dont,
}
impl From<&str> for Modifier {
    fn from(value: &str) -> Self {
        let value: &str = &value.to_lowercase();
        match value {
            "don't()" => Modifier::Dont,
            "do()" => Modifier::Do,
            "none" | "" | _ => Modifier::None,
        }
    }
}
impl<'a, T> From<Option<T>> for Modifier
where
    T: Into<&'a str>,
{
    fn from(val: Option<T>) -> Self {
        match val {
            Some(x) => Modifier::from(x.into()),
            None => Modifier::from(""),
        }
    }
}
fn part2(input: &str) {
    // run the regex
    // get the last modifier
    // if the last modifier is don't then skip
    // if the last modifier is do or none then skip
    let re = Regex::new(r"((?<modifier>do(n't)?\(\)).*?)?mul\((?<x>\d+),(?<y>\d+)\)").unwrap();
    let mut last_modifier: Modifier = Modifier::Do; // start with a do to allow lines when a modifier isn't present
    let ops: Vec<MultOps> = re
        .captures_iter(input)
        .filter_map(|cap| {
	    let temp_modified = cap.name("modifier").into(); 
            match temp_modified { // if modifier changed and isn't none then update last modified 
		Modifier::Dont => last_modifier = Modifier::Dont,
		Modifier::Do => last_modifier = Modifier::Do,
		Modifier::None => (),
	    };
            let (x, y): (Num, Num) = (
                cap.name("x")?.as_str().parse::<Num>().ok()?,
                cap.name("y")?.as_str().parse::<Num>().ok()?,
            );
            info!(target:"part2","Last Mod:{:?}\tCurr Mod:{:?}\tOperands({x},{y})",last_modifier,temp_modified);
            if last_modifier == Modifier::Do {
                return Some(MultOps::new(x, y));
            }
            None
        })
        .collect();
    println!(
        "Total: {}",
        ops.iter().fold(0, |total, op| total + op.calc())
    );
}
fn main() {
    env_logger::init();
    //part1(include_str!("../files/input"));
    part2(include_str!("../files/input"));
}

#[test]
fn test_part1() {
    env_logger::init();
    part1(include_str!("../files/test"))
}

#[test]
fn test_part2() {
    env_logger::init();
    part2(include_str!("../files/test"))
}

#[test]
fn test_enum() {
    env_logger::init();
    assert_eq!(Modifier::None, Modifier::from(""));
    assert_eq!(Modifier::Do, Modifier::from("do()"));
    assert_eq!(Modifier::Dont, Modifier::from("don't()"));
}
