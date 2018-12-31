extern crate s0rt;

use s0rt::algorithms::{
    stalin_sort,
    bogo_sort,
    sleep_sort
};

fn create_dummy_list() -> Vec<String> {
    vec!["7", "1", "5", "7", "22"]
        .into_iter()
        .map(String::from)
        .collect()
}

#[test]
fn test_stalin_sort() {
    let list = create_dummy_list();
    let tests = vec![
        ((false, false), vec!["7", "7"]),
        ((true, false), vec!["7", "1"]),
        ((false, true), vec!["7", "7", "22"]),
        ((true, true), vec!["7", "1"]),
    ];

    for (args, label) in tests.into_iter() {
        assert_eq!(stalin_sort(&list, args.0, args.1), label);
    }
}

#[test]
fn test_bogo_sort() {
    let list = create_dummy_list();
    let tests = vec![
        ((false, false), vec!["1", "22", "5", "7", "7"]),
        ((true, false), vec!["7", "7", "5", "22", "1"]),
        ((false, true), vec!["1", "5", "7", "7", "22"]),
        ((true, true), vec!["22", "7", "7", "5", "1"]),
    ];

    for (args, label) in tests.into_iter() {
        assert_eq!(bogo_sort(&list, args.0, args.1), label);
    }
}

#[test]
fn test_sleep_sort() {
    let list = create_dummy_list();
    assert_eq!(sleep_sort(&list), vec!["1", "5", "7", "7", "22"]);
}
