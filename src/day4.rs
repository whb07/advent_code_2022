use std::collections::HashSet;
use std::str::FromStr;

fn create_assignment(a: &str) -> HashSet<u8> {
    let nums: Vec<&str> = a.split("-").collect();
    let range = |n: Vec<&str>| -> Vec<u8> {
        (u8::from_str(n[0]).unwrap()..u8::from_str(n[1]).unwrap() + 1).collect()
    };
    HashSet::from_iter(range(nums).into_iter())
}

fn parse_assignments(data: &Vec<String>) -> Vec<(HashSet<u8>, HashSet<u8>)> {
    let mut result = Vec::new();
    for pair in data.iter() {
        let ass: Vec<&str> = pair.split(",").collect();
        result.push((create_assignment(ass[0]), create_assignment(ass[1])));
    }
    result
}

fn is_superset(s: &(HashSet<u8>, HashSet<u8>)) -> bool {
    let (a, b) = s;
    a.is_superset(b) || b.is_superset(a)
}

fn intersects(s: &(HashSet<u8>, HashSet<u8>)) -> bool {
    let (a, b) = s;
    a.intersection(b).any(|e| *e > 0)
}

fn day_4_solution_part1(data: &Vec<String>) -> usize {
    let result: Vec<(HashSet<u8>, HashSet<u8>)> = parse_assignments(data)
        .into_iter()
        .filter(is_superset)
        .collect();
    result.len()
}

fn day_4_solution_part2(data: &Vec<String>) -> usize {
    let result: Vec<(HashSet<u8>, HashSet<u8>)> = parse_assignments(data)
        .into_iter()
        .filter(intersects)
        .collect();
    result.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_file;

    #[test]
    fn test_parse_assignments() {
        let data = vec!["2-4,6-8".to_string()];
        let result = parse_assignments(&data);
        let expected: Vec<(HashSet<u8>, HashSet<u8>)> = vec![(
            HashSet::from_iter(vec![2u8, 3u8, 4u8].into_iter()),
            HashSet::from_iter(vec![6u8, 7u8, 8u8].into_iter()),
        )];
        assert_eq!(expected, result);
    }

    #[test]
    fn test_day4() {
        let data =
            read_file("/home/wbright/projects/advent_of_code_2022/input/day4.txt".to_string());
        let result = day_4_solution_part1(&data);

        assert_eq!(475, result);

        let result2 = day_4_solution_part2(&data);
        assert_eq!(825, result2);
    }
}
