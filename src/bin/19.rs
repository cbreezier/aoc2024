use std::collections::HashMap;
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "19";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn can_make(question: &str, towels: &Vec<String>, index: usize, cache: &mut HashMap<usize, bool>) -> bool {
        if cache.contains_key(&index) {
            return cache[&index];
        }
        if index >= question.len() {
            return true;
        }
        for towel in towels {
            let cur_q = &question[index..];
            if cur_q.starts_with(towel) {
                if can_make(question, towels, index + towel.len(), cache) {
                    cache.insert(index, true);
                    return true;
                }
            }
        }
        cache.insert(index, false);
        false
    }

    fn part1<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut all_input = String::new();
        reader.read_to_string(&mut all_input)?;

        let (towels_str, questions_str) = all_input.split_once("\n\n").unwrap();
        let towels = towels_str.split(", ")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let questions = questions_str.trim().split("\n")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let answer = questions.iter()
            .filter(|q| {
                let mut cache = HashMap::new();
                let result = can_make(&q, &towels, 0, &mut cache);
                result
            })
            .count();
        Ok(answer)
    }

    assert_eq!(6, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    
    fn num_ways(question: &str, towels: &Vec<String>, index: usize, cache: &mut HashMap<usize, usize>) -> usize {
        if cache.contains_key(&index) {
            return cache[&index];
        }
        if index >= question.len() {
            return 1;
        }
        let mut answer = 0usize;
        for towel in towels {
            let cur_q = &question[index..];
            if cur_q.starts_with(towel) {
                answer += num_ways(question, towels, index + towel.len(), cache);
            }
        }
        cache.insert(index, answer);
        answer
    }
    
    fn part2<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut all_input = String::new();
        reader.read_to_string(&mut all_input)?;

        let (towels_str, questions_str) = all_input.split_once("\n\n").unwrap();
        let towels = towels_str.split(", ")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let questions = questions_str.trim().split("\n")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let answer = questions.iter()
            .map(|q| {
                let mut cache = HashMap::new();
                let result = num_ways(&q, &towels, 0, &mut cache);
                result
            })
            .sum();
        Ok(answer)
    }
    
    assert_eq!(16, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
