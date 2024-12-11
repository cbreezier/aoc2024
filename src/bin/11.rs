use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "11";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
125 17
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn num_digits(value: usize) -> usize {
        let mut digits = 1usize;
        let mut power = 10usize;
        while power <= value {
            power *= 10;
            digits += 1;
        }

        digits
    }

    fn pow(base: usize, exponent: usize) -> usize {
        let mut result = 1;
        for _ in 0..exponent {
            result *= base;
        }
        result
    }

    fn num_stones(value: usize, steps: usize) -> usize {
        // println!("Computing {} at step {}", value, steps);
        if steps == 0 {
            return 1;
        }

        if value == 0 {
            return num_stones(1, steps - 1);
        }

        let digits = num_digits(value);
        // println!("  {} digits", digits);
        if digits % 2 == 0 {
            let half_power = pow(10, digits / 2);
            // println!("  {} half power", half_power);
            let left = value / half_power;
            let right = value % half_power;
            // println!("  Trying {} and {}", left, right);
            return num_stones(left, steps - 1) + num_stones(right, steps - 1);
        }

        num_stones(value * 2024, steps - 1)
    }

    fn part1<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut line = String::new();
        reader.read_line(&mut line)?;

        let input: Vec<usize> = line.trim().split(" ").map(|x| x.parse::<usize>().unwrap()).collect();

        let answer = input.iter().map(|value| num_stones(*value, 25)).sum();

        Ok(answer)
    }

    assert_eq!(55312, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn num_stones2(value: usize, steps: usize, cache: &mut HashMap<(usize, usize), usize>) -> usize {
        // println!("Computing {} at step {}", value, steps);
        if steps == 0 {
            return 1;
        }
        
        match cache.get(&(value, steps)) {
            Some(result) => return *result,
            None => (),
        }

        if value == 0 {
            let result = num_stones2(1, steps - 1, cache);
            cache.insert((value, steps), result);
            return result;
        }

        let digits = num_digits(value);
        // println!("  {} digits", digits);
        if digits % 2 == 0 {
            let half_power = pow(10, digits / 2);
            // println!("  {} half power", half_power);
            let left = value / half_power;
            let right = value % half_power;
            // println!("  Trying {} and {}", left, right);
            let result = num_stones2(left, steps - 1, cache) + num_stones2(right, steps - 1, cache);
            cache.insert((value, steps), result);
            return result;
        }

        let result = num_stones2(value * 2024, steps - 1, cache);
        cache.insert((value, steps), result);
        result
    }
    
    fn part2<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut line = String::new();
        reader.read_line(&mut line)?;

        let input: Vec<usize> = line.trim().split(" ").map(|x| x.parse::<usize>().unwrap()).collect();

        let mut cache: HashMap<(usize, usize), usize> = HashMap::new();
        let answer = input.iter().map(|value| num_stones2(*value, 75, &mut cache)).sum();

        Ok(answer)
    }
    
    // assert_eq!(81, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
