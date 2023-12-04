use std::collections::HashMap;

fn main() {
    let input = include_str!("input2.txt");
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
    ]
    .iter()
    .cloned()
    .collect();

    for line in input.lines() {
        let mut first_occurrence = line.len(); // Initialize to a value larger than any possible index
        let mut last_occurrence = 0;
        let mut first_keyword = "";
        let mut last_keyword = "";

        for keyword in keywords.iter().chain(digits.iter()) {
            if let Some(start) = line.find(keyword) {
                if start < first_occurrence {
                    first_occurrence = start;
                    first_keyword = keyword;
                    println!("{}", first_occurrence);
                    println!("{}", first_keyword);
                }
            
                if start > last_occurrence {
                    last_occurrence = start;
                    last_keyword = keyword;
                    println!("{}", last_occurrence);
                    println!("{}", last_keyword);
                }
            }
        }

        total += map.get(first_keyword).unwrap_or(&0) + map.get(last_keyword).unwrap_or(&0);
    }

    println!("{}", total);
}
