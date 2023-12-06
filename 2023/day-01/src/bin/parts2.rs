use std::collections::HashMap;
use regex::Regex; 

fn main() {
    let input = include_str!("input1.txt");
    let keywords = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let digits = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let mut total = 0;
    let map: HashMap<&str, i32> = [
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]
    .iter()
    .cloned()
    .collect();
    for line in input.lines() {
        let mut first_occurrence = line.len(); 
        let mut last_occurrence = 0;
        let mut first_keyword = "";
        let mut last_keyword = "";

        for keyword in keywords.iter().chain(digits.iter()) {
            let v: Vec<_> = line.match_indices(keyword).collect(); 

            if !v.first().is_none() {
                let f = v.first().unwrap().0;
                if f <= first_occurrence {
                    first_occurrence = f;
                    first_keyword = keyword;
                }
            } else {
                continue;
            }
            let mut potential_last = 0; 
            if !v.last().is_none() {
                potential_last = v.last().unwrap().0;
            } else if !v.first().is_none() {
                potential_last = v.first().unwrap().0;
            }
            if potential_last >= last_occurrence {
                last_occurrence = potential_last;
                last_keyword = keyword;
            }

        }
        let line_total = 10*map.get(first_keyword).unwrap_or(&0) + map.get(last_keyword).unwrap_or(&0);
        total += line_total
    }
    println!("{}", total);
}
