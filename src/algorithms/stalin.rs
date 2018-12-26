pub fn sort(v: &Vec<String>, reverse: bool) -> Vec<String> {
    let mut sorted = v.clone();
    let mut count = 0;
    for (i, a) in v.iter().skip(1).enumerate() {
        let b = &sorted[i - count];
        if (!reverse && a < b) || (reverse && a > b) {
            sorted.remove(i - count + 1);
            count += 1;
        }
    }

    sorted
}
