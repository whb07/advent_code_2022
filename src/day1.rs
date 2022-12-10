use crate::read_file;

pub fn parse_calories(mut data: Vec<String>) -> Vec<Vec<usize>> {
    let mut res = Vec::new();
    res.push(Vec::new());
    while !data.is_empty() {
        let item = data.pop().unwrap();
        if item.is_empty() {
            res.push(Vec::new())
        } else {
            let last = res.last_mut();
            match last {
                Some(n) => n.push(item.parse::<usize>().unwrap()),
                None => (),
            }
        }
    }
    res
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn first_day() -> usize {
    let data = read_file(
        "/home/wbright/storage_shared/projects/advent_of_code_2022/input/day1.txt".to_string(),
    );
    let mut result: Vec<usize> = parse_calories(data)
        .iter()
        .map(|e| e.iter().sum())
        .collect();
    result.sort();
    result.iter().rev().take(3).sum()
}
