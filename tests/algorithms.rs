extern crate s0rt;

use s0rt::algorithms::{stalin, bogo};

fn create_dummy() -> Vec<String> {
    vec!["7", "1", "5", "7"].into_iter().map(String::from).collect()
}

#[test]
fn test_stalin_sort() {
    let dummy = create_dummy();

    assert_eq!(stalin::sort(&dummy, false), vec!["7", "7"]);
    assert_eq!(stalin::sort(&dummy, true), vec!["7", "1"]);
}

#[test]
fn test_bogo_sort() {
    let dummy = create_dummy();

    assert_eq!(bogo::sort(&dummy, false), vec!["1", "5", "7", "7"]);
    assert_eq!(bogo::sort(&dummy, true), vec!["7", "7", "5", "1"]);
}
