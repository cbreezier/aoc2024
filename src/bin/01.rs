use std::collections::HashMap;
use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use scan_fmt::{scan_fmt_some};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::zip;

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<u32> {
        let input = reader.lines()
            .map(|l| -> (u32, u32) {
                let (a, b) = scan_fmt_some!(&l.unwrap(), "{d}  {d}", u32, u32);
                (a.unwrap(), b.unwrap())
            })
            .collect::<Vec<(u32, u32)>>();
        let arr1 = input.iter()
            .map(|(a, _)| a)
            .sorted()
            .collect::<Vec<&u32>>();
        let arr2 = input.iter()
            .map(|(_, b)| b)
            .sorted()
            .collect::<Vec<&u32>>();
        let answer = zip(arr1, arr2)
            .map(|(a, b)| a.abs_diff(*b))
            .sum::<u32>();
        Ok(answer)
    }

    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<u32> {
        let input = reader.lines()
            .map(|l| -> (u32, u32) {
                let (a, b) = scan_fmt_some!(&l.unwrap(), "{d}  {d}", u32, u32);
                (a.unwrap(), b.unwrap())
            })
            .collect::<Vec<(u32, u32)>>();

        // Count number of occurences of each number in the second column
        let arr2_counts = input.iter()
            .map(|(_, b)| b)
            .fold(HashMap::new(), |mut counts, val| {
                counts.entry(val)
                    .and_modify(|v| *v += 1)
                    .or_insert(1u32);
                counts
            });

        // For each number in the first column, multiply by its count in the second column then sum
        let answer = input.iter()
            .map(|(a, _)| a)
            .map(|a| a * arr2_counts.get(a).unwrap_or(&0u32))
            .sum::<u32>();
        Ok(answer)
    }

    assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
