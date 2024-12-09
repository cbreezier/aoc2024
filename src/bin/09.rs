use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use adv_code_2024::*;

const DAY: &str = "09";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
2333133121414131402
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn gauss_sum(start: usize, len: usize) -> usize {
        let end = start + len - 1;
        ((start + end) * len) / 2
    }

    fn part1<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut line: String = String::new();
        reader.read_line(&mut line)?;
        let input = line.trim().chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect::<Vec<usize>>();

        let mut file_index = 0;
        let mut i = 0usize;
        let mut end_file_index = if input.len() % 2 == 0 {
            (input.len() - 2) / 2
        } else {
            (input.len() - 1) / 2
        };
        let mut end_file_consumed = 0usize;
        let mut answer = 0usize;
        while file_index * 2 < input.len() && file_index <= end_file_index {
            // Block
            let file_len = input[file_index * 2];
            // println!("Processing block of {} at index {}", file_len, file_index);
            if file_index == end_file_index {
                answer += file_index * gauss_sum(i, file_len - end_file_consumed);
                // println!("  Indices crossing over at {}, consuming {} of {} remaining blocks", file_index, file_len - end_file_consumed, file_len);
                break;
            }
            answer += file_index * gauss_sum(i, file_len);
            i += file_len;

            // Gap
            if file_index * 2 + 1 < input.len() {
                let gap_len = input[file_index * 2 + 1];
                // println!("Processing gap of {}", gap_len);
                let mut remaining_blocks = gap_len;
                while remaining_blocks > 0 {
                    let end_file_len = input[end_file_index * 2] - end_file_consumed;
                    // println!("  There are {} blocks of {} we can use", end_file_len, end_file_index);
                    if end_file_len >= remaining_blocks {
                        // println!("  Consuming all {} blocks of {}", remaining_blocks, end_file_index);
                        answer += end_file_index * gauss_sum(i, remaining_blocks);
                        i += remaining_blocks;
                        end_file_consumed += remaining_blocks;
                        remaining_blocks = 0;
                    } else {
                        // println!("  Consuming {} blocks of {}", end_file_len, end_file_index);
                        answer += end_file_index * gauss_sum(i, end_file_len);
                        i += end_file_len;
                        end_file_index -= 1;
                        end_file_consumed = 0;
                        remaining_blocks -= end_file_len;
                    }
                }
            }

            file_index += 1;
        }
        Ok(answer)
    }

    assert_eq!(1928, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    #[derive(Copy, Clone)]
    struct DiskBlock {
        is_file: bool, // otherwise is gap
        file_index: usize,
        length: usize,
    }
    
    fn print_blocks(blocks: &Vec<DiskBlock>) {
        for block in blocks {
            if block.is_file {
                print!("{}", vec![block.file_index.to_string(); block.length].iter().join(""));
            } else {
                print!("{}", vec!["."; block.length].iter().join(""));
            }
        }
        println!();
    }

    fn part2<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut line: String = String::new();
        reader.read_line(&mut line)?;
        let input = line.trim().chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect::<Vec<usize>>();

        let mut blocks = input.iter()
            .enumerate()
            .map(|(i, &length)| {
                if i % 2 == 0 {
                    DiskBlock{is_file: true, file_index: i / 2, length}
                } else {
                    DiskBlock{is_file: false, file_index: 0, length}
                }
            })
            .collect::<Vec<DiskBlock>>();
        
        // print_blocks(&blocks);

        for end_block_index in (0..blocks.len()).rev() {
            let block = blocks[end_block_index];
            if !block.is_file {
                continue;
            }

            for gap_index in 0..end_block_index {
                let gap = blocks[gap_index];
                if gap.is_file {
                    continue;
                }
                if gap.length >= block.length {
                    blocks[end_block_index].is_file = false;
                    blocks[gap_index].length -= block.length;
                    blocks.insert(gap_index, block);

                    break;
                }
            }
            
            // print_blocks(&blocks);
        }

        let mut i = 0usize;
        let mut answer = 0usize;
        for block in blocks {
            if block.is_file {
                answer += block.file_index * gauss_sum(i, block.length);
                i += block.length;
            } else {
                i += block.length;
            }
        }

        Ok(answer)
    }

    assert_eq!(2858, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
