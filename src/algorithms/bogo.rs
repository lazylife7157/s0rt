extern crate rand;

use std::cmp::Ordering::{self, Greater};
use rand::thread_rng;
use rand::seq::SliceRandom;

fn is_sorted(v: &Vec<String>, cmp: &Box<Fn(&String, &String) -> Ordering>) -> bool {
    for i in 1..v.len() {
        if cmp(&v[i - 1], &v[i]) == Greater {
            return false;
        }
    }

    true
}

pub fn sort(v: &Vec<String>, cmp: &Box<Fn(&String, &String) -> std::cmp::Ordering>) -> Vec<String> {
    let mut sorted = v.clone();
	let mut rng = thread_rng();

    while !is_sorted(&sorted, cmp) {
		sorted.shuffle(&mut rng);
    }

	sorted
}
