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
const TEST2: &str = "\
111: 1 1 1
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
    println!("\n=== Part 2 ===");

    // Given 6, return 10. Given 76, return 100
    fn next_base_10(num: usize) -> usize {
        let mut result = 1;
        while result <= num {
            result *= 10;
        }
        result
    }

    fn can_make2(result: usize, values: &Vec<usize>, index: i32) -> bool {
        // let indent = vec![" "; (values.len() as i32 - index) as usize].join("");
        if result == 0 {
            // println!("{}Success", indent);
            return true;
        }
        if index < 0 {
            // println!("{}Failed", indent);
            return false;
        }
        // println!("{}Make {} from {}?", indent, result, values.iter().take((index + 1) as usize).join(","));
        let cur = values[index as usize];
        if cur <= result {
            // println!("{}Try +", indent);
            if can_make2(result - cur, values, index - 1) {
                return true;
            }
        }
        // Bit convoluted, but the idea is that if we need to make 486 and we're currently 6, check that
        // we can make 48 with the remaining numbers (and that 6 matches the end digits of course)
        let base_if_concat = next_base_10(cur);
        // println!("{}Base is {} and cur is {} and result is {}", indent, base_if_concat, cur, result);
        if base_if_concat < result && result % base_if_concat == cur {
            // println!("{}Try ||", indent);
            if can_make2(result / base_if_concat, values, index - 1) {
                return true;
            }
        }
        if result % cur == 0 {
            // println!("{}Try *", indent);
            if can_make2(result / cur, values, index - 1) {
                return true;
            }
        }
        false
    }

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
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

                if can_make2(result, &values, values.len() as i32 - 1) {
                    Some(result)
                } else {
                    None
                }
            })
            .sum();
        Ok(answer)
    }

    assert_eq!(11387, part2(BufReader::new(TEST.as_bytes()))?);
    assert_eq!(111, part2(BufReader::new(TEST2.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
