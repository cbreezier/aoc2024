use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use adv_code_2024::*;

const DAY: &str = "07";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");
    
    fn can_make(result: usize, values: &Vec<usize>, index: i32) -> bool {
        if result == 0 {
            return true;
        }
        if index < 0 {
            return false;
        }
        let cur = values[index as usize];
        (cur <= result && can_make(result - cur, values, index - 1)) || (result % cur == 0 && can_make(result / cur, values, index - 1))
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let answer = reader.lines()
            .filter_map(|l| {
                let line = l.unwrap();
                let mut parts = line.split(' ');
                // Extract the {result}: at the start
                let result = parts.next().unwrap() // First part
                    .chars()
                    .dropping_back(1) // Drop the : at the end
                    .as_str()
                    .parse::<usize>().unwrap();
                // We've already consumed the first part, the rest are the values
                let values = parts.map(|n| n.parse::<usize>().unwrap()).collect::<Vec<usize>>();
                
                // println!("Trying to make {} from {}", result, values.iter().join(","));
                
                if can_make(result, &values, values.len() as i32 - 1) {
                    Some(result)
                } else {
                    None
                }
            })
            .sum();
        Ok(answer)
    }

    assert_eq!(3749, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");
    //
    // fn part2<R: BufRead>(reader: R) -> Result<usize> {
    //     Ok(0)
    // }
    //
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);
    //endregion

    Ok(())
}
