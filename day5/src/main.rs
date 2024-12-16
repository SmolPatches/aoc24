use std::collections::{binary_heap::Iter, HashMap};
use std::iter::IntoIterator;
fn part1(input: &str) {
    // split page rules and updates
    let mut page_rules: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut i = input.lines().into_iter(); // create an iterator from the input
    i.by_ref()
        .take_while(|line| !line.trim().is_empty())
        .for_each(|line| {
            println!("Lines: {line}");
            let (lhs, rhs) = line.split_once("|").unwrap();
            println!("LHS:{lhs},RHS:{rhs}");
            page_rules
                .entry(lhs.parse().ok().unwrap())
                .or_insert_with(Vec::new)
                .push(rhs.parse().ok().unwrap());
        });
    println!("HMAP:{:?}", page_rules);
    let updates: Vec<Vec<usize>> = i
        .map(|line| Vec::from_iter(line.split(",").filter_map(|item| item.parse().ok()))) // get all the updates from file
        .filter(|update: &Vec<usize>| {
            let mut v = update.iter();
            // add first elm to v
            let mut temp = v.next().unwrap();
            v.for_each(|num| if page_rules.get(num).and_then(|rules| rules.contains(temp) {
                return false;
            }) );
            true
        })
        .collect();
    println!("Updates:{:?}", updates);
}
fn main() {
    part1(include_str!("../files/input"));
}
#[test]
fn d5_p1() {
    part1(include_str!("../files/test"));
    println!("{:?}", HashMap::from([(0, 9), (1, 8), (2, 7)]));
}
