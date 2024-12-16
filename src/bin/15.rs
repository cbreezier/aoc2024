use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "15";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";

// In order of "turning right"
// (0, 0) is the top left
const DIRS: [[i32; 2]; 4] = [
    [1, 0],
    [0, 1],
    [-1, 0],
    [0, -1],
];

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");
    
    fn try_move(map: &mut Vec<Vec<u8>>, pos: (i32, i32), dir_index: usize) -> bool {
        let height = map.len();
        let width = map[0].len();
        
        let cur_pos_item = map[pos.1 as usize][pos.0 as usize];
        let dir = DIRS[dir_index];
        let new_pos = (pos.0 + dir[0], pos.1 + dir[1]);
        
        // Bounds check
        if new_pos.0 < 0 || new_pos.0 >= width as i32 || new_pos.1 < 0 || new_pos.1 >= height as i32 {
            return false;
        }
        
        let new_pos_item = map[new_pos.1 as usize][new_pos.0 as usize];
        if new_pos_item == b'#' {
            false
        } else if new_pos_item == b'O' {
            if try_move(map, new_pos, dir_index) {
                map[new_pos.1 as usize][new_pos.0 as usize] = cur_pos_item;
                map[pos.1 as usize][pos.0 as usize] = b'.';
                true
            } else {
                false
            }
        } else {
            map[new_pos.1 as usize][new_pos.0 as usize] = cur_pos_item;
            map[pos.1 as usize][pos.0 as usize] = b'.';
            true
        }
    }
    
    fn print_map(map: &Vec<Vec<u8>>) {
        let height = map.len();
        let width = map[0].len();

        for y in 0..height {
            for x in 0..width {
                print!("{}", char::from(map[y][x]));
            }
            println!();
        }
    }

    fn part1<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut all_input = String::new();
        reader.read_to_string(&mut all_input)?;
        
        let (map_input, moves_input) = all_input.split_once("\n\n").unwrap();
        let mut map = map_input.split("\n")
            .map(|l| l.chars().map(|c| c as u8).collect())
            .collect::<Vec<Vec<u8>>>();
        let moves = moves_input.chars()
            .filter_map(|c| {
                match c {
                    '>' => Some(0usize),
                    'v' => Some(1usize),
                    '<' => Some(2usize),
                    '^' => Some(3usize),
                    _ => None,
                }
            })
            .collect::<Vec<usize>>();

        let height = map.len();
        let width = map[0].len();
        
        let mut initial_pos = (0, 0);
        for y in 0..height {
            for x in 0..width {
                if map[y][x] == b'@' {
                    initial_pos = (x as i32, y as i32);
                }
            }
        }
        
        let mut pos = initial_pos;
        for dir_index in moves {
            // println!("Moving in dir {}", dir_index);
            if try_move(&mut map, pos, dir_index) {
                pos = (pos.0 + DIRS[dir_index][0], pos.1 + DIRS[dir_index][1]);
            }
            // print_map(&map);
        }
        
        print_map(&map);
        
        let mut result = 0usize;
        for y in 0..height {
            for x in 0..width {
                if map[y][x] == b'O' {
                    result += y * 100 + x;
                }
            }
        }

        Ok(result)
    }

    assert_eq!(10092, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn try_move2(map: &mut Vec<Vec<u8>>, pos: (i32, i32), dir_index: usize, dry_run: bool) -> bool {
        let height = map.len();
        let width = map[0].len();

        let cur_pos_item = map[pos.1 as usize][pos.0 as usize];
        let dir = DIRS[dir_index];
        let new_pos = (pos.0 + dir[0], pos.1 + dir[1]);

        // Bounds check
        if new_pos.0 < 0 || new_pos.0 >= width as i32 || new_pos.1 < 0 || new_pos.1 >= height as i32 {
            return false;
        }

        let new_pos_item = map[new_pos.1 as usize][new_pos.0 as usize];
        if new_pos_item == b'#' {
            false
        } else if (new_pos_item == b'[' || new_pos_item == b']') && (dir_index == 0 || dir_index == 2) {
            // Pushing wide boxes horizontally is the same as before
            if try_move2(map, new_pos, dir_index, false) {
                if !dry_run {
                    map[new_pos.1 as usize][new_pos.0 as usize] = cur_pos_item;
                    map[pos.1 as usize][pos.0 as usize] = b'.';
                }
                true
            } else {
                false
            }
        } else if new_pos_item == b'[' && (dir_index == 1 || dir_index == 3) {
            // Pushing left side of wide boxes vertically
            let box_right = (new_pos.0 + 1, new_pos.1);
            if try_move2(map, new_pos, dir_index, true) && try_move2(map, box_right, dir_index, true) {
                if !dry_run {
                    try_move2(map, new_pos, dir_index, false);
                    try_move2(map, box_right, dir_index, false);
                    map[new_pos.1 as usize][new_pos.0 as usize] = cur_pos_item;
                    map[pos.1 as usize][pos.0 as usize] = b'.';
                }
                true
            } else {
                false
            }
        } else if new_pos_item == b']' && (dir_index == 1 || dir_index == 3) {
            // Pushing right side of wide boxes vertically
            let box_left = (new_pos.0 - 1, new_pos.1);
            if try_move2(map, new_pos, dir_index, true) && try_move2(map, box_left, dir_index, true) {
                if !dry_run {
                    try_move2(map, new_pos, dir_index, false);
                    try_move2(map, box_left, dir_index, false);
                    map[new_pos.1 as usize][new_pos.0 as usize] = cur_pos_item;
                    map[pos.1 as usize][pos.0 as usize] = b'.';
                }
                true
            } else {
                false
            }
        } else {
            if !dry_run {
                map[new_pos.1 as usize][new_pos.0 as usize] = cur_pos_item;
                map[pos.1 as usize][pos.0 as usize] = b'.';
            }
            true
        }
    }

    fn part2<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut all_input = String::new();
        reader.read_to_string(&mut all_input)?;

        let (map_input, moves_input) = all_input.split_once("\n\n").unwrap();
        let mut map = map_input.split("\n")
            .map(|l| l.chars().map(|c| {
                match c {
                    '#' => [b'#', b'#'],
                    'O' => [b'[', b']'],
                    '@' => [b'@', b'.'],
                    '.' => [b'.', b'.'],
                    _ => unreachable!(),
                }
            }).flatten().collect())
            .collect::<Vec<Vec<u8>>>();
        let moves = moves_input.chars()
            .filter_map(|c| {
                match c {
                    '>' => Some(0usize),
                    'v' => Some(1usize),
                    '<' => Some(2usize),
                    '^' => Some(3usize),
                    _ => None,
                }
            })
            .collect::<Vec<usize>>();

        let height = map.len();
        let width = map[0].len();
        
        let mut initial_pos = (0, 0);
        for y in 0..height {
            for x in 0..width {
                if map[y][x] == b'@' {
                    initial_pos = (x as i32, y as i32);
                }
            }
        }

        let mut pos = initial_pos;
        for dir_index in moves {
            // println!("Moving in dir {}", dir_index);
            if try_move2(&mut map, pos, dir_index, false) {
                pos = (pos.0 + DIRS[dir_index][0], pos.1 + DIRS[dir_index][1]);
            }
            // print_map(&map);
        }

        print_map(&map);

        let mut result = 0usize;
        for y in 0..height {
            for x in 0..width {
                if map[y][x] == b'[' {
                    result += y * 100 + x;
                }
            }
        }

        Ok(result)
    }

    assert_eq!(9021, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
