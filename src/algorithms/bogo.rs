extern crate rand;

use rand::thread_rng;
use rand::seq::SliceRandom;

use crate::algorithms::{key, is_sorted_by};

pub fn bogo_sort<T>(list: &Vec<T>, reverse: bool, numeric: bool) -> Vec<T>
        where T: ToString + Clone {

    let mut sorted = list.clone();
    let mut rng = thread_rng();

    while !is_sorted(&sorted, reverse, numeric) {
        sorted.shuffle(&mut rng);
    };

    sorted
}

fn is_sorted<T>(list: &Vec<T>, reverse: bool, numeric: bool) -> bool
        where T: ToString + Clone {

    if numeric {
        is_sorted_by(list, key::as_numeric, reverse)
    } else {
        is_sorted_by(list, key::as_string, reverse)
    }
}
