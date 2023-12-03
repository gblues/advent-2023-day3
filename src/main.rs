
use regex::Regex;

fn main() {
    let gear_pattern = Regex::new(r"\*").unwrap();
    let num_pattern = Regex::new(r"\d+").unwrap();
    let input: &str = include_str!("input/3.txt");
    let data = if input.ends_with("\n") { input.strip_suffix("\n").unwrap() } else {input};
    let mut total: i32 = 0;
    let lines:Vec<_> = data.split("\n").collect();

    for (row, line) in lines.iter().enumerate() {
        for g in gear_pattern.find_iter(*line) {
            let min: usize = if g.range().start > 0 {g.range().start -1} else {0};
            let max: usize = g.range().end;
            let mut lines_to_search: Vec<&str> = Vec::new();

            if row > 0 { lines_to_search.push(lines[row-1]);}
            lines_to_search.push(*line);
            if row < lines.len()-1 { lines_to_search.push(lines[row+1]);}
            let mut part_numbers: Vec<i32> = Vec::new();

            for search in lines_to_search {
                num_pattern.find_iter(search).filter(
                    |n| (n.range().end > min) && (n.range().start <= max)
                ).for_each(|part| part_numbers.push(part.as_str().parse::<i32>().unwrap())
                );
            }
            if part_numbers.len() == 2 {
                total += part_numbers.iter().product::<i32>();
            }
        }
    }
    println!("{}", total);
}
