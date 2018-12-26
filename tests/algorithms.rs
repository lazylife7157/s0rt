extern crate s0rt;

use s0rt::algorithms::stalin;

#[test]
fn test_stalin_sort() {
    let dummy = vec!["7", "1", "5", "7"].into_iter().map(String::from).collect();

    assert_eq!(stalin::sort(&dummy, false), vec!["7", "7"]);
    assert_eq!(stalin::sort(&dummy, true), vec!["7", "1"]);
}
