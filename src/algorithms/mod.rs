use std::cmp::Ordering::{self, Less, Greater};

pub use self::stalin::stalin_sort;
pub use self::bogo::bogo_sort;
pub use self::sleep::sleep_sort;

mod stalin;
mod bogo;
mod sleep;

mod key {
    pub fn as_string<T>(x: &T) -> String where T: ToString {
        x.to_string()
    }

    pub fn as_numeric<T>(x: &T) -> u64 where T: ToString {
        x.to_string().parse::<u64>().unwrap_or(0)
    }
}

pub fn cmp_by<T, F, K>(a: &T, b: &T, key_fn: F) -> Ordering
        where F: Fn(&T) -> K, K: Ord {

    key_fn(a).cmp(&key_fn(b))
}

pub fn is_sorted_by<T, F, K>(list: &Vec<T>, key_fn: F, reverse: bool) -> bool
        where F: Fn(&T) -> K, K: Ord {

    let order = if reverse { Less } else { Greater };

    for i in 1..list.len() {
        let a = &list[i - 1];
        let b = &list[i];
        if cmp_by(a, b, &key_fn) == order {
            return false;
        }
    }

    true
}
