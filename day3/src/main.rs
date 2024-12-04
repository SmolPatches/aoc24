use env_logger;
use log::{self, info};
use regex::Regex;
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
            Some((x, y))
        })
        .map(|v| MultOps::new(v.0, v.1))
        .collect();
    println!(
        "Total: {}",
        ops.iter().fold(0, |total, op| total + op.calc())
    );
}
fn main() {
    env_logger::init();
    part1(include_str!("../files/input"));
}

#[test]
fn test_part1() {
    env_logger::init();
    part1(include_str!("../files/test"))
}
