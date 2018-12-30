use std::cmp::Ordering;
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

pub fn sort(v: &Vec<String>, _: &Box<Fn(&String, &String) -> Ordering>) -> Vec<String> {
    let (tx, rx) = mpsc::channel();

    for s in v.clone().into_iter() {
        let tx0 = tx.clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(match s.parse::<u64>() {
                Ok(t) => t,
                _ => 1
            }));
            tx0.send(s).expect("Failed to send message");
        });
    }

    (0..v.len())
        .filter_map(|_| rx.recv().ok())
        .collect()
}
