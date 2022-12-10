use crate::read_file;
use std::collections::HashSet;

fn split_compartments(data: &String) -> (HashSet<char>, HashSet<char>) {
    fn hasher(n: &str) -> HashSet<char> {
        HashSet::from_iter(n.chars())
    }
    let (head, tail) = data.split_at(data.len() / 2);
    (hasher(head), hasher(tail))
}

fn priority_value(c: &char) -> usize {
    fn lowercase(n: &char) -> usize {
        u8::try_from(n.to_owned()).unwrap() as usize - 96
    }
    fn uppercase(n: &char) -> usize {
        u8::try_from(n.to_owned()).unwrap() as usize - 38
    }
    match c {
        'a'..='z' => lowercase(c),
        _ => uppercase(c),
    }
}

fn arrange(data: &String) -> usize {
    let (head, tail) = split_compartments(data);
    let found = head.intersection(&tail);
    let mut score = 0;
    for c in head.intersection(&tail) {
        let points = priority_value(c);
        score += points;
    }
    score
}

pub fn day_3(path: String) -> usize {
    let data: Vec<String> = read_file(path)
        .iter()
        .filter(|e| !e.is_empty())
        .map(|e| e.to_owned())
        .collect();
    data.iter().map(arrange).sum()
}
