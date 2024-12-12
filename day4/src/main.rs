use env_logger;
use log::info;
use std::mem;
use std::ops::Add;

// directions in a 2d array
#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    LDiag,
    RDiag,
    LDiagUp,
    RDiagUp,
}
impl Direction {
    fn to_offset(&self) -> Offset {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
            Direction::LDiagUp => (-1, -1),
            Direction::LDiag => (1, -1),
            Direction::RDiag => (1, 1),
            Direction::RDiagUp => (-1, 1),
        }
    }
    fn to_position(&self, pos: Position) -> Option<Position> {
        pos + self.to_offset()
    }
}
impl Add<Offset> for Position {
    type Output = Option<Position>;
    fn add(self, other: Offset) -> Option<Self> {
        let row = self.row as isize + other.0 as isize;
        let col = self.col as isize + other.1 as isize;

        if row >= 0 && col >= 0 {
            Some(Self {
                row: row as usize,
                col: col as usize,
            })
        } else {
            None
        }
    }
}
//type Position = (usize, usize);
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Position {
    row: usize,
    col: usize,
}
impl Position {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}
// impl fmt::Display for Position {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "({},{})", self.row, self.col)
//     }
// }
static mut recursion_count: usize = 0;
static mut flag: bool = false;
type Offset = (i32, i32);
fn part1(input: &str) {
    // call this on every positon
    // this should return a set of positons( as steps) for each pos to find the match
    // could return none
    let stack: Vec<char> = "XMAS".chars().collect();
    fn string_match_handler(
        pos: Position,
        v: &Vec<Vec<char>>,
        stack: &[char], // this might need to be a Rope or an Rc since further members might consume a slice
    ) -> Option<Vec<Vec<Position>>> {
        // helper function for matches
        fn string_matches(
            pos: Position,
            v: &Vec<Vec<char>>,
            stack: &[char], // this might need to be a Rope or an Rc since further members might consume a slice
            direction: Direction,
            //pairs: &mut Vec<Position>,
            pairs: &mut Vec<Position>, // return type should be a hash set of positions (since each position must be unique) in the match
        ) -> Option<Vec<Position>> {
            if !(pos.row < v.len() && pos.col < v.len()) {
                return None;
            }
            let curr_char = v[pos.row][pos.col];
            if stack.len() == 1 && stack[0] == curr_char {
                // this means last letter in stack is a match
                // this is a full match
                // here we should return all the ordered pairs we discovered
                //info!(target:"end recursion","found a match:{:?}",pairs);
                info!(target:"end recursion","found a match:{:?} in {:?}",pairs,direction);
                return Some(mem::take(pairs));
            }
            if curr_char != stack[0] {
                info!(
                    "Found {} in ({},{}) was expecting {}",
                    curr_char, pos.row, pos.col, stack[0]
                );
                return None;
            }
            pairs.push(pos);
            let pos = direction.to_position(pos)?;
            string_matches(pos, v, &stack[1..], direction, pairs)
            // change the position based on direction
            // and call again
        }
        // 2. get window / screen of available direction
        // 2.1 check if pos in vec
        // 2
        let directions: [Direction; 8] = [
            // i could make a macro for this to make an iterator over enum variants
            // good way to potentially work with const / generics
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
            Direction::LDiag,
            Direction::LDiagUp,
            Direction::RDiag,
            Direction::RDiagUp,
        ];
        // slice of all the vectors of matches
        let set = Vec::from_iter(directions.into_iter().filter_map(|direction| {
            //let mut dir_set = HashSet::new();
            let mut dir_set = Vec::new();
            //let pos = direction.to_position(pos)?;
            string_matches(
                pos, // skip negative usize positions
                &v,
                &stack,
                direction,
                &mut dir_set,
            )
        }));
        if set.len() == 0 {
            return None;
        }
        Some(set)
    }
    let xmas_vec: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    xmas_vec.iter().for_each(|line| println!("{:?}", line));
    let mut set: Vec<Vec<Position>> = Vec::new();
    // is there a rustier way to do this?
    // note there is prob an algo to do this w/ a flat map
    // idea: instead of screen positions being (0,+1) for left, diagnol being (+1,+1) we could do some other way
    for row in 0..xmas_vec.len() {
        for col in 0..xmas_vec.len() {
            let pos_matches = string_match_handler(Position::new(row, col), &xmas_vec, &stack);
            if let Some(match_set) = pos_matches {
                set.extend(match_set)
            }
        }
    }
    set.iter().for_each(|l| info!(target:"set_print","{:?}", l));
    println!("total matches:{}", set.len());
}
fn main() {
    env_logger::init();
    part1(include_str!("../files/input"))
}
#[test]
fn test4_part1() {
    env_logger::init();
    part1(include_str!("../files/test"));
}

#[test]
fn test4_direction() {
    env_logger::init();
    let x = Position { row: 0, col: 0 };
    let y: Offset = (-1, 0);
    assert_eq!(x + y, None);
    let x = Position { row: 1, col: 0 };
    let y: Offset = (-1, 0);
    assert_ne!(x + y, None);
    let x = Position { row: 9, col: 9 };
    let y = x + Direction::LDiagUp.to_offset();
    assert_eq!(y.unwrap(), Position { row: 8, col: 8 });
    let x = Position { row: 9, col: 9 };
    let y = x + Direction::LDiagUp.to_offset();
    assert_eq!(y.unwrap(), Position { row: 8, col: 8 });
}
//.map(|match_vec| HashSet::from_iter(match_vec))
