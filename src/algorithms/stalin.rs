use std::cmp::Ordering::{self, Greater};

pub fn sort(v: &Vec<String>, cmp: &Box<Fn(&String, &String) -> Ordering>) -> Vec<String> {
    let mut sorted = v.clone();
    let mut count = 0;
    for (i, b) in v.iter().skip(1).enumerate() {
        let a = &sorted[i - count];
        if cmp(a, b) == Greater {
            sorted.remove(i - count + 1);
            count += 1;
        }
    }

    sorted
}
