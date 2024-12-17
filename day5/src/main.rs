use std::collections::HashMap;
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
        .enumerate()
        .filter(|(line_num, update)| {
            // filter remove bad arrays
            //for each element of update
            //check if curr elm is a key
            //if last elm is inside the mapping for current key then we have bad news
            // stop checking array once we find a rule break
            // return otherwise
            if update.len() == 0 {
                return false; // skip empty lines in updates
            }
            let mut v = update.iter();
            let mut is_valid: bool = true;
            let mut temp = v
                .next()
                .expect(format!("Failed on vec: {:?} @ Line: {line_num}", update).as_str());
            v.for_each(|num| {
                if matches!(
                    page_rules.get(num).map(|rules| rules.contains(temp)),
                    Some(true)
                ) {
                    // if order is wrong
                    is_valid = false;
                    return (); // match page rules violated
                }
                temp = num
            });
            is_valid
        })
        .map(|(_, update)| update)
        .collect();
    println!("Updates:{:?}", updates);
    let sum = updates.iter().fold(0, |acc, update| {
        // get middle element and add it to accumulator
        update
            .get(update.len() / 2)
            .expect("Couldn't get middle element")
            + acc
    });
    println!("Middle Sum: {sum}")
}
fn main() {
    part1(include_str!("../files/input"));
}
#[test]
fn d5_p1() {
    part1(include_str!("../files/test"));
    println!("{:?}", HashMap::from([(0, 9), (1, 8), (2, 7)]));
}
