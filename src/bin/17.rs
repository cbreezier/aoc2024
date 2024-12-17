use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use scan_fmt::scan_fmt_some;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "17";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";
const TEST2: &str = "\
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");
    
    fn run_program(r_init: (usize, usize, usize), program: &Vec<usize>) -> String {
        let mut r = r_init;
        let mut ip = 0usize;
        
        let mut output: Vec<usize> = Vec::new();
        
        loop {
            // Account for the operand as well
            if ip >= program.len() - 1 {
                break;
            }
            
            let opcode = program[ip];
            let operand = program[ip + 1];
            let combo_operand = match operand {
                0 => Some(0),
                1 => Some(1),
                2 => Some(2),
                3 => Some(3),
                4 => Some(r.0),
                5 => Some(r.1),
                6 => Some(r.2),
                _ => None,
            };
            
            match opcode {
                0 => { // adv
                    r.0 = r.0 >> combo_operand.unwrap();
                    ip += 2;
                }
                1 => { // bxl
                    r.1 = r.1 ^ operand;
                    ip += 2;
                }
                2 => { // bst
                    r.1 = combo_operand.unwrap() % 8;
                    ip += 2;
                }
                3 => { // jnz
                    if r.0 != 0 {
                        ip = operand;
                    } else {
                        ip += 2;
                    }
                }
                4 => { // bxc
                    r.1 = r.1 ^ r.2;
                    ip += 2;
                }
                5 => { // out
                    output.push(combo_operand.unwrap() % 8);
                    ip += 2;
                }
                6 => { // bdv
                    r.1 = r.0 >> combo_operand.unwrap();
                    ip += 2;
                }
                7 => { // cdv
                    r.2 = r.0 >> combo_operand.unwrap();
                    ip += 2;
                }
                _ => panic!("Unknown opcode {} at ip {}", opcode, ip),
            }
        }
        
        output.iter().join(",").to_string()
    }

    fn part1<R: BufRead>(mut reader: R) -> Result<String> {
        let mut all_input = String::new();
        reader.read_to_string(&mut all_input)?;

        let (registers_input, program_input) = all_input.split_once("\n\n").unwrap();
        let registers_input_lines = registers_input.split("\n").collect::<Vec<&str>>();
        let r_a = scan_fmt_some!(registers_input_lines[0], "Register A: {d}", usize);
        let r_b = scan_fmt_some!(registers_input_lines[1], "Register B: {d}", usize);
        let r_c = scan_fmt_some!(registers_input_lines[2], "Register C: {d}", usize);

        let program = program_input
            .split(" ")
            .last()
            .unwrap()
            .trim()
            .split(",")
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        Ok(run_program((r_a.unwrap(), r_b.unwrap(), r_c.unwrap()), &program))
    }

    assert_eq!("4,6,3,5,6,3,5,2,1,0", part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    
    // Returns true if the last [index] outputs correspond to the last [index] program instructions.
    fn run_program2(r_init: (usize, usize, usize), program: &Vec<usize>, index: usize) -> bool {
        let mut r = r_init;
        let mut ip = 0usize;

        let mut output: Vec<usize> = Vec::new();

        loop {
            // Account for the operand as well
            if ip >= program.len() - 1 {
                break;
            }

            let opcode = program[ip];
            let operand = program[ip + 1];
            let combo_operand = match operand {
                0 => Some(0),
                1 => Some(1),
                2 => Some(2),
                3 => Some(3),
                4 => Some(r.0),
                5 => Some(r.1),
                6 => Some(r.2),
                _ => None,
            };

            match opcode {
                0 => { // adv
                    r.0 = r.0 >> combo_operand.unwrap();
                    ip += 2;
                }
                1 => { // bxl
                    r.1 = r.1 ^ operand;
                    ip += 2;
                }
                2 => { // bst
                    r.1 = combo_operand.unwrap() % 8;
                    ip += 2;
                }
                3 => { // jnz
                    if r.0 != 0 {
                        ip = operand;
                    } else {
                        ip += 2;
                    }
                }
                4 => { // bxc
                    r.1 = r.1 ^ r.2;
                    ip += 2;
                }
                5 => { // out
                    output.push(combo_operand.unwrap() % 8);
                    ip += 2;
                }
                6 => { // bdv
                    r.1 = r.0 >> combo_operand.unwrap();
                    ip += 2;
                }
                7 => { // cdv
                    r.2 = r.0 >> combo_operand.unwrap();
                    ip += 2;
                }
                _ => panic!("Unknown opcode {} at ip {}", opcode, ip),
            }
        }
        
        if output.len() != program.len() {
            panic!("Wrong output length entirely...");
        }
        // println!("Output of {}", output.iter().join(","));
        
        let len = program.len();
        for i in (len - 1 - index)..len {
            if program[i] != output[i] {
                return false;
            }
        }
        true
    }

    // The top 3 bytes influence the last output of the program. The next 3 bytes influence the
    // second last output, so on and so forth.
    //
    // Given an existing `a` and an index (from the end of the program), modify `a`
    // in such a way that the nth 3 bytes produce the correct output.
    fn find_quine(r_init: (usize, usize, usize), program: &Vec<usize>, index: usize) -> Option<usize> {
        if index >= program.len() {
            return Some(r_init.0);
        }

        // println!("Finding quine from {:b} at index {}", a_init, index);
        let clear_bits = !(0b111usize << ((program.len() - 1 - index) * 3));
        for i in 0..8usize {
            if index == 0 && i == 0 {
                continue;
            }
            
            let set_bits = i << ((program.len() - 1 - index) * 3);
            let a = (r_init.0 & clear_bits) | set_bits;
            // println!("  Trying {:b}", a);
            if run_program2((a, r_init.1, r_init.2), program, index) {
                match find_quine((a, r_init.1, r_init.2), program, index + 1) {
                    Some(a) => return Some(a),
                    None => {}
                }
            }
        }
        None
    }
    
    fn part2<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut all_input = String::new();
        reader.read_to_string(&mut all_input)?;

        let (registers_input, program_input) = all_input.split_once("\n\n").unwrap();
        let registers_input_lines = registers_input.split("\n").collect::<Vec<&str>>();
        let _ = scan_fmt_some!(registers_input_lines[0], "Register A: {d}", usize);
        let r_b = scan_fmt_some!(registers_input_lines[1], "Register B: {d}", usize);
        let r_c = scan_fmt_some!(registers_input_lines[2], "Register C: {d}", usize);

        let program_str = program_input
            .split(" ")
            .last()
            .unwrap()
            .trim();
        let program = program_str
            .split(",")
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let result = find_quine((0, r_b.unwrap(), r_c.unwrap()), &program, 0);
        result.ok_or(Error::msg("No solution found."))
    }

    assert_eq!(117440, part2(BufReader::new(TEST2.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
