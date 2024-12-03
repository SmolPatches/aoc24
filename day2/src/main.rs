use env_logger;
use log::{debug, info};
type Num = i32;
fn main() {
    env_logger::init();
    let f: Vec<Vec<Num>> = include_str!("../files/input")
        .lines()
	// map each line to a vector of numbers
        .map(|line| {
            line.split_ascii_whitespace()
                .filter_map(|word| word.parse::<Num>().ok()) // turn lines into scanable numbers
                .collect()
        })
	// check if it is ascending or descending order
        .filter(|array: &Vec<Num>| {
	    array.is_sorted_by(|a,b| a>=b) || array.is_sorted_by(|a,b| a<=b) 
	})
        .filter(|array| {// return arrays where the scan is equal to original array length 
            array.iter().scan(-1, |state, i| {
		if *state==-1 { // autopass first state
		    *state=i-1;
		}
                let calc = Num::abs_diff(*state, *i);
		*state = *i;
                if calc < 1 || calc > 3 { // make sure we aren't in start state
		    return None
                }
		Some(*state)
            }).count() == array.len()
        }).collect();
    println!("Len:{:?}", f.len());
}
