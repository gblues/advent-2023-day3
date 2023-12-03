
use regex::Regex;
// 012345678
// abcdef123
// start: 6
// end: 9
fn main() {
    let pattern = Regex::new(r"\d+").unwrap();
    let data: &str = include_str!("input/3.txt").strip_suffix("\n").unwrap();

    let grid:Vec<_> = data
        .split("\n")
        .map(|line| line.chars().collect::<Vec<_>>()).collect();
    let mut total: i32 = 0;

    let lines:Vec<_> = data.split("\n").collect();
    for (row, line) in lines.iter().enumerate() {
        for m in pattern.find_iter(*line) {
            let begin_x: usize =  if m.range().start > 0 {m.range().start-1} else { 0 };
            let begin_y: usize = if row > 0 {row-1} else {0};
            println!("len: {}, end: {}", (*line).len(), m.range().end);
            let end_x: usize = if m.range().end == (*line).len() {line.len()-1} else {m.range().end};
            let end_y: usize = if row+1 == lines.len() {row} else {row+1};
            println!("found: {} at ({}, {}). searching ({}, {}) to ({}, {})",
                     m.as_str(), m.range().start, row, begin_x, begin_y, end_x, end_y);
            let mut is_part_number:bool = false;

            for x in begin_x..=end_x {
                for y in begin_y..=end_y {
                    match grid[y][x] {
                        '.' => {},
                        '0'..='9' => {},
                        _ => is_part_number = true
                    }
                }
            }
            if is_part_number {
                total += m.as_str().parse::<i32>().unwrap();
            }
        }
    }
    println!("{}", total);
}
