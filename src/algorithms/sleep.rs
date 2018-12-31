use std::thread;
use std::sync::mpsc;
use std::time::Duration;

pub fn sleep_sort<T>(list: &Vec<T>) -> Vec<T>
        where T: 'static + ToString + Clone + Send {

    let (tx, rx) = mpsc::channel();

    for s in list.clone().into_iter() {
        let tx0 = tx.clone();
        thread::spawn(move || {
            let millis = s.to_string().parse::<u64>().unwrap_or(1);
            let duration = Duration::from_millis(millis);
            thread::sleep(duration);
            tx0.send(s).expect("Failed to send message");
        });
    }

    (0..list.len())
        .filter_map(|_| rx.recv().ok())
        .collect()
}
