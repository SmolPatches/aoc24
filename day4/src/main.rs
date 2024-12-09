use env_logger;
use log::info;
use std::collections::HashSet;
use std::mem;
use std::ops::Add;

// directions in a 2d array
enum Direction {
    Up,
    Down,
    Left,
    Right,
    LDiag,
    RDiag,
}
impl Direction {
    fn to_offset(&self) -> Offset {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
            Direction::LDiag => (1, -1),
            Direction::RDiag => (1, 1),
        }
    }
    fn to_position(self, pos: Position) -> Position {
        pos + self.to_offset()
    }
}
impl Add<Offset> for Position {}
type Position = (usize, usize);
type Offset = (i32, i32);
fn part1(input: &str) {
    // call this on every positon
    // this should return a set of positons( as steps) for each pos to find the match
    // could return none
    let stack: Vec<char> = "xmas".chars().collect();
    fn string_match_handler(
        pos: Position,
        v: &Vec<Vec<char>>,
        stack: &[char], // this might need to be a Rope or an Rc since further members might consume a slice
    ) -> Option<HashSet<HashSet<Position>>> {
        // helper function for matches
        let mut left_set: Vec<Position>;
        let mut right_set: Vec<Position>;
        let mut up_set: Vec<Position>;
        let mut down_set: Vec<Position>;
        let mut left_diag_set: Vec<Position>;
        let mut right_diag_set: Vec<Position>;
        fn string_matches(
            pos: Position,
            v: &Vec<Vec<char>>,
            stack: &[char], // this might need to be a Rope or an Rc since further members might consume a slice
            direction: Direction,
            pairs: &mut Vec<Position>,
            // return type should be a hash set of positions (since each position must be unique) in the match
        ) -> Option<Vec<Position>> {
            // 1. check if stack matches
            // if stack doesn't match then return None
            // this is one end condition of the recursion
            // TODO: maybe use and then?
            // or check validity in range in the recursive calls below
            if v[pos.0][pos.1] != stack[0] {
                return None;
            }
            if stack.len() == 0 {
                // this is a full match
                // here we should return all the ordered pairs we discovered
                return Some(mem::take(pairs));
            }
            // else call again with same direction
            //NOTE: need some way to track previous positions
            // pairs vec
            pairs.push(pos);
            let pos = pos + direction.to_offset(); // make this valid(impl add<Offset> for Postion)
            string_matches(pos, v, &stack[1..], direction, pairs) // recursion
                                                                  // change the position based on direction
                                                                  // and call again
        }
        // 2. get window / screen of available direction
        // 2.1 check if pos in vec
        // 2
        let directions: [Direction; 6] = [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
            Direction::LDiag,
            Direction::RDiag,
        ];
        // slice of all the vectors of matches
        return directions.map(|direction| {
            let mut dir_set = Vec::new();
            string_matches(
                Direction::to_position(direction, pos),
                &v,
                &stack[1..],
                direction,
                &mut dir_set,
            )
        }); // convert this to a set of a set of positions
    }
    let xmas_vec: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    xmas_vec
        .iter()
        .for_each(|v: &Vec<char>| println!(/*target:"vec printer",*/ "{:?}", &v));
    let xmas_matches: HashSet<HashSet<Position>>;
    // is there a rustier way to do this?
    // note there is prob an algo to do this w/ a flat map
    // idea: instead of screen positions being (0,+1) for left, diagnol being (+1,+1) we could do some other way
    for x in 0..xmas_vec.len() {
        for y in 0..xmas_vec.len() {
            let pos_matches = string_match_handler((x, y), &xmas_vec, &stack);
            // matches expression? if its not none append to xmas_matches
        }
    }
}
fn main() {
    env_logger::init();
    part1(include_str!("../files/input"))
}
#[test]
fn test4_part1() {
    env_logger::init();
    part1(include_str!("../files/test"))
}
