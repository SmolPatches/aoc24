use log::info;
use std::collections::HashMap;
fn bubble_sort(page_rules: &HashMap<usize, Vec<usize>>, src: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut src = src;
    src.iter_mut()
        .map(|arr| -> Vec<usize> {
            for _ in 0..arr.len() {
                let mut swap = false;
                for i in 0..arr.len() - 1 {
                    if matches!(
                        page_rules.get(&arr[i + 1]).map(|row| row.contains(&arr[i])),
                        Some(true)
                    ) {
                        // swap
                        arr.swap(i, i + 1);
                        swap = true;
                    }
                }
                if !swap {
                    break;
                }
            }
            return arr.to_vec();
        })
        .collect()
}
fn get_updates(input: &str, good: bool) -> Vec<Vec<usize>> {
    // split page rules and updates
    let mut page_rules: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut i = input.lines().into_iter(); // create an iterator from the input
    i.by_ref()
        .take_while(|line| !line.trim().is_empty())
        .for_each(|line| {
            info!("Lines: {line}");
            let (lhs, rhs) = line.split_once("|").unwrap();
            info!("LHS:{lhs},RHS:{rhs}");
            page_rules
                .entry(lhs.parse().ok().unwrap())
                .or_insert_with(Vec::new)
                .push(rhs.parse().ok().unwrap());
        });
    info!("HMAP:{:?}", page_rules);
    let final_vec: Vec<Vec<usize>> = i
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
            let mut is_valid: bool = good; // return good or bads
            let mut temp = v
                .next()
                .expect(format!("Failed on vec: {:?} @ Line: {line_num}", update).as_str());
            v.for_each(|num| {
                if matches!(
                    page_rules.get(num).map(|rules| rules.contains(temp)),
                    Some(true)
                ) {
                    // if order is wrong
                    is_valid = !good;
                    return (); // match page rules violated
                }
                temp = num
            });
            is_valid
        })
        .map(|(_, update)| update)
        .collect();
    if !good {
        bubble_sort(&page_rules, final_vec)
    } else {
        final_vec
    }
}
fn main() {
    // part 1
    let sum = get_updates(include_str!("../files/input"), true)
        .iter()
        .fold(0, |acc, update| {
            // get middle element and add it to accumulator
            update
                .get(update.len() / 2)
                .expect("Couldn't get middle element")
                + acc
        });
    println!("Middle Sum: {sum}");
    // part2
    let sum = get_updates(include_str!("../files/input"), false)
        .iter()
        .fold(0, |acc, update| {
            // get middle element and add it to accumulator
            update
                .get(update.len() / 2)
                .expect("Couldn't get middle element")
                + acc
        });
    println!("Middle Sum: {sum}");
}
#[test]
fn d5_p1() {
    get_updates(include_str!("../files/test"), true);
}

#[test]
fn d5_p2() {
    get_updates(include_str!("../files/test"), false);
}
