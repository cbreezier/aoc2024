use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
";
const TEST2: &str = "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(mut reader: R) -> Result<u64> {
        let mut input = String::new();
        reader.read_to_string(&mut input)?;

        let mut result = 0u64;
        let regex = Regex::new(r"mul\((\d+),(\d+)\)")?;
        for (_, [a, b]) in regex.captures_iter(&input).map(|c| c.extract()) {
            result += a.parse::<u64>()? * b.parse::<u64>()?;
        }

        Ok(result)
    }

    assert_eq!(161, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(mut reader: R) -> Result<u64> {
        let mut input = String::new();
        reader.read_to_string(&mut input)?;

        let instruction_regex = Regex::new(r"(mul\(\d+,\d+\))|(do\(\))|(don't\(\))")?;
        let mul_regex = Regex::new(r"mul\((\d+),(\d+)\)")?;

        let mut result = 0u64;
        let mut mul_enabled = true;
        for (_, [instruction]) in instruction_regex.captures_iter(&input).map(|c| c.extract()) {
            if instruction == "do()" {
                mul_enabled = true;
            } else if instruction == "don't()" {
                mul_enabled = false;
            } else if instruction.starts_with("mul") && mul_enabled {
                let (_, [a, b]) = mul_regex.captures(&instruction).unwrap().extract();
                result += a.parse::<u64>()? * b.parse::<u64>()?;
            }
        }

        Ok(result)
    }
    
    assert_eq!(48, part2(BufReader::new(TEST2.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
