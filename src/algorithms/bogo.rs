extern crate rand;

use rand::thread_rng;
use rand::seq::SliceRandom;

fn is_sorted(v: &Vec<String>, reverse: bool) -> bool {
    for i in 1..v.len() {
        if (!reverse && v[i - 1] > v[i]) || (reverse && v[i - 1] < v[i]) {
            return false;
        }
    }
    true
}

pub fn sort(v: &Vec<String>, reverse: bool) -> Vec<String> {
    let mut sorted = v.clone();
	let mut rng = thread_rng();

    while !is_sorted(&sorted, reverse) {
		sorted.shuffle(&mut rng);
    }

	sorted
}
