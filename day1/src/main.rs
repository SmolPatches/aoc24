use std::collections::BinaryHeap;
use std::cmp::Reverse;
type Num = i32;
use log::info;
fn main() {
    env_logger::init();
    let (heap1,heap2): (BinaryHeap<Reverse<Num>>, BinaryHeap<Reverse<Num>>) = std::fs::read_to_string("files/input").unwrap().lines().map(|line| {
	    let (num1,num2) = line.split_once("   ").unwrap();
	    let (num1,num2) = (num1.parse::<Num>().unwrap(), num2.parse::<Num>().unwrap());
	    (Reverse(num1),Reverse(num2))
	}).collect();
    assert_eq!(heap1.len(),heap2.len()); // do some fun stuff to move this to a type check if i make it a function
    /*
	let distances: Vec<u32> = heap1.into_iter().zip(heap2.into_iter()).map(|(a,b)| {
     * Note while this seems natural / correct
	into_iter returns in arbitrary order
	so i need to make it a sorted vec, then into an iterator
	then reverse it
    Oh the pain
    Check out resources:
	https://users.rust-lang.org/t/surprising-behavior-of-binaryheap-into-iter/17315
	https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html#method.into_iter_sorted
	https://github.com/rust-lang/rust/issues/59278
	https://github.com/rust-lang/rust/issues/76250
    */
    let distances: Vec<u32> = heap1.into_sorted_vec().into_iter().rev().zip(heap2.into_sorted_vec().into_iter().rev()).map(|(a,b)| {
	Num::abs_diff(a.0,b.0) // returns u32( because diff ≥ 0 )
    }).collect();
    distances.iter().enumerate().for_each(|(i,v)| info!(target:"Distance Print","Distance {i}: {v}"));
    let total: u32 = distances.into_iter().sum();
    println!("Sum: {total}");
}
