use std::collections::HashSet;
use std::fs;

fn read_file(path: &str) -> String {
    fs::read_to_string(path).expect("Something went wrong reading file")
}

fn find_start_marker(data:&String, target:usize) -> usize {

    for current in 0..data.len() {
        let mut result = HashSet::new();
        let slice = &data[current..];
        for (index, c) in slice.char_indices() {
            if result.len() == target -1 {
                if result.insert(c) {
                    return index + current + 1
                } else {
                    break;
                }
            } else {
                if !result.insert(c) {
                    break;
                }
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_start_marker() {
        assert_eq!(7, find_start_marker(&"mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string(), 4));
        assert_eq!(5, find_start_marker(&"bvwbjplbgvbhsrlpgdmjqwftvncz".to_string(), 4));
        assert_eq!(6, find_start_marker(&"nppdvjthqldpwncqszvftbrmjlhg".to_string(), 4));
        assert_eq!(10, find_start_marker(&"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string(), 4));
        assert_eq!(11, find_start_marker(&"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string(), 4));
    }

    #[test]
    fn test_part_1() {
        assert_eq!(1779, find_start_marker(&read_file("/home/wbright/projects/advent_of_code_2022/input/day6.txt"), 4));
    }
}