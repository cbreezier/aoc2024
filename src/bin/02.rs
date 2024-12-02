use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn is_safe(input: &Vec<u32>) -> bool {
        let mut deltas: Vec<i32> = Vec::new();
        for (index, num) in input.iter().enumerate() {
            if index == 0 {
                continue;
            }
            deltas.push((*num as i32) - (input[index - 1] as i32));
        }

        deltas.iter().all(|x| *x < 0 && *x >= -3)
            || deltas.iter().all(|x| *x > 0 && *x <= 3)
    }

    fn part1<R: BufRead>(reader: R) -> Result<u32> {
        let input = reader.lines()
            .map(|l| {
                l.unwrap()
                    .split(" ")
                    .map(|s| s.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect::<Vec<Vec<u32>>>();

        let num_safe = input.iter().filter(|x| is_safe(x)).count() as u32;
        Ok(num_safe)
    }

    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<u32> {
        let input = reader.lines()
            .map(|l| {
                l.unwrap()
                    .split(" ")
                    .map(|s| s.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect::<Vec<Vec<u32>>>();

        let num_safe = input.iter()
            .filter(|x| {
                // We're now allowed to remove one item from the input - brute force try removing
                // each item and see if any of the resulting inputs are now "safe"
                for i in 0..x.len() {
                    let sliced_input = [&x[0..i], &x[i + 1..]].concat();
                    if is_safe(&sliced_input) {
                        return true;
                    }
                }
                return false;
            })
            .count() as u32;
        Ok(num_safe)
    }
    
    assert_eq!(4, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
