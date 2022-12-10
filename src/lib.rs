mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

use std::fs;

pub fn read_file(path: String) -> Vec<String> {
    let res: Vec<String> = fs::read_to_string(path)
        .unwrap()
        .split("\n")
        .map(|e| e.to_string())
        .collect();
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::day1::{first_day, parse_calories};
    use crate::day2::*;
    use crate::day3::*;

    #[test]
    fn test_parse_calories() {
        assert_eq!(vec![vec![1usize]], parse_calories(vec!["1".to_string()]));
        assert_eq!(
            vec![vec![2usize], vec![1usize]],
            parse_calories(vec!["1".to_string(), "".to_string(), "2".to_string()])
        );
        assert_eq!(1, first_day());
    }

    #[test]
    fn test_day2_score() {
        let expected = Round {
            hero: Hand::Rock,
            opponent: Hand::Rock,
        };
        // assert_eq!(expected, Round::from(&"A X".to_string()));
        let data =
            read_file("/home/wbright/projects/advent_of_code_2022/input/day2.txt".to_string());
        // assert_eq!(expected.score(), 4);
        assert_eq!(12, run_game(&data));
    }

    #[test]
    fn test_day3_score() {
        assert_eq!(
            12,
            day_3("/home/wbright/projects/advent_of_code_2022/input/day3.txt".to_string())
        );
    }
}
