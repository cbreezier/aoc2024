use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "05";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let lines = reader.lines().collect::<Result<Vec<String>, _>>()?;
        let rules = lines
            .iter()
            .filter_map(|l| {
                if l.contains('|') {
                    let items = l
                        .split('|')
                        .map(|s| s.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>();
                    return Some([items[0], items[1]]);
                }
                return None;
            })
            .collect::<Vec<[usize; 2]>>();

        let answer = lines
            .iter()
            .filter_map(|l| {
                return if l.contains(',') {
                    Some(
                        l.split(',').map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>()
                    )
                } else {
                    None
                }
            })
            .filter_map(|input| {
                if rules.iter().all(|rule| {
                    let pos1 = input.iter().find_position(|it| rule[0] == **it);
                    let pos2 = input.iter().find_position(|it| rule[1] == **it);
                    return match (pos1, pos2) {
                        (Some(pos1), Some(pos2)) => {
                            pos1 < pos2
                        }
                        _ => {
                            true
                        },
                    }
                }) {
                    Some(input[input.len() / 2])
                } else {
                    None
                }
            })
            .sum();
        Ok(answer)
    }

    assert_eq!(143, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let lines = reader.lines().collect::<Result<Vec<String>, _>>()?;
        let rules = lines
            .iter()
            .filter_map(|l| {
                if l.contains('|') {
                    let items = l
                        .split('|')
                        .map(|s| s.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>();
                    return Some([items[0], items[1]]);
                }
                return None;
            })
            .collect::<Vec<[usize; 2]>>();

        let answer = lines
            .iter()
            .filter_map(|l| {
                return if l.contains(',') {
                    Some(
                        l.split(',').map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>()
                    )
                } else {
                    None
                }
            })
            // Same as part 1
            .filter(|input| {
                if rules.iter().all(|rule| {
                    let pos1 = input.iter().find_position(|it| rule[0] == **it);
                    let pos2 = input.iter().find_position(|it| rule[1] == **it);
                    return match (pos1, pos2) {
                        (Some(pos1), Some(pos2)) => {
                            pos1 < pos2
                        }
                        _ => {
                            true
                        },
                    }
                }) {
                    false
                } else {
                    // Keep only the unsorted ones
                    true
                }
            })
            .map(|input| {
                // Kahn's algorithm (topsort)
                // The graph is the list of edges that are relevant to our input
                let mut graph = rules.iter()
                    .filter(|rule| input.contains(&rule[0]) && input.contains(&rule[1]))
                    .collect::<Vec<_>>();
                let mut result: Vec<usize> = Vec::new();
                // Set of nodes without incoming vertices
                let mut s: Vec<usize> = input.iter()
                    .map(|i| *i)
                    .filter(|i| graph.iter().all(|rule| rule[1] != *i))
                    .collect();

                while !s.is_empty() {
                    let n = s.pop().unwrap();
                    result.push(n);
                    // M's to check later (an edge e goes from n to m)
                    let mut ms: Vec<usize> = Vec::new();
                    graph.retain(|edge| {
                        return if edge[0] == n {
                            let m = edge[1];
                            ms.push(m);
                            // Delete all edges that originate from our popped node n
                            false
                        } else {
                            true
                        }
                    });

                    ms.iter()
                        .map(|m| *m)
                        .filter(|m| graph.iter().all(|edge| edge[1] != *m))
                        .for_each(|m| s.push(m));
                }

                // println!("{} unsorted, sorted into {}", input.iter().join(","), result.iter().join(","));

                result[result.len() / 2]
            })
            .sum();

        Ok(answer)
    }

    assert_eq!(123, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
