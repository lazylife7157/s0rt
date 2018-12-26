use std::cmp::Ordering;

pub mod stalin;
pub mod bogo;

pub fn get_compare_fn(reverse: bool) -> Box<Fn(&String, &String) -> Ordering> {
    if reverse {
        Box::new(|a, b| a.cmp(b).reverse())
    } else {
        Box::new(|a, b| a.cmp(b))
    }
}
