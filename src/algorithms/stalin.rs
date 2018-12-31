use std::cmp::Ordering::{Less, Greater};

use crate::algorithms::{key, cmp_by};

pub fn stalin_sort<T>(list: &Vec<T>, reverse: bool, numeric: bool) -> Vec<T>
        where T: ToString + Clone {

    let order = if reverse { Less } else { Greater };
    let mut sorted = list.clone();
    let mut i = 1;

    while i < sorted.len() {
        let a = &sorted[i - 1];
        let b = &sorted[i];
        let is_dead = if numeric {
            cmp_by(a, b, key::as_numeric)
        } else {
            cmp_by(a, b, key::as_string)
        } == order;

        if is_dead {
            sorted.remove(i);
        } else {
            i += 1;
        }
    }

    sorted
}
