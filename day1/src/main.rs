use std::cmp::Reverse;
use std::collections::BinaryHeap;
type Num = i32;
use log::info;
fn main() {
    env_logger::init();
    let (heap1, heap2): (BinaryHeap<Reverse<Num>>, BinaryHeap<Reverse<Num>>) =
        std::fs::read_to_string("files/input")
            .unwrap()
            .lines()
            .map(|line| {
                let (num1, num2) = line.split_once("   ").unwrap();
                let (num1, num2) = (num1.parse::<Num>().unwrap(), num2.parse::<Num>().unwrap());
                (Reverse(num1), Reverse(num2))
            })
            .collect();
    let (mut vec1, vec2) = (heap1.into_sorted_vec(), heap2.into_sorted_vec());
    let distances: Vec<u32> = vec1
        .iter()
        .rev()
        .zip(vec2.iter().rev())
        .map(|(a, b)| {
            Num::abs_diff(a.0, b.0) // returns u32( because diff ≥ 0 )
        })
        .collect();
    distances
        .iter()
        .enumerate()
        .for_each(|(i, v)| info!(target:"Distance Print","Distance {i}: {v}"));
    let total: u32 = distances.into_iter().sum();
    println!("Sum: {total}");
    // Day 1 P 2
    let similarity_score: usize = {
        vec1.dedup();
        vec1.iter()
            .map(|num| {
                // for each number in the left list get the amount of times its in right list, then multiply it by num.0
                num.0 as usize * vec2.iter().filter(|num2| num.0 == num2.0).count()
            })
            .sum() // is there a better way to do this?
    };
    println!("Similarity Score: {similarity_score}");
}
