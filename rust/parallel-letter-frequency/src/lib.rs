use std::collections::HashMap;
use std::thread;
use std::sync::Arc;

pub fn frequency (text: &[&str], n: usize) -> HashMap<char, usize> {
    let data: Vec<String> = text.iter().map(|s| s.to_lowercase()).collect();
    let arc = Arc::new(data);
    let mut threads = vec![];

    for i in 0 .. n {
        let data = arc.clone();
        threads.push(thread::spawn(move || {
        let mut counts = HashMap::new();
        let mut k = i;
        while k < data.len() {
            for c in data.get(k).unwrap().chars().filter(|c| c.is_alphabetic()) {
            *counts.entry(c).or_insert(0) += 1;
            }
            k += n;
        }
        counts
        }));
    }

    let mut counts: HashMap<char, usize> = HashMap::new();
    for thread in threads {
        for (k, v) in thread.join().unwrap() {
        *counts.entry(k).or_insert(0) += v;
        }
    }
    counts
}