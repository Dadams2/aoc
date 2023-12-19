use std::collections::HashMap;

const RED: i32 = 12;
const GREEN: i32 = 13;
const BLUE: i32 = 14;


#[derive(Debug, PartialEq, Eq, Hash)]
enum Color {
    Red,
    Green,
    Blue,
}

fn parse_line(line: &str) -> HashMap<Color, i32> {
    let mut result = HashMap::new();

    // Split the line into individual segments
    let segments: Vec<&str> = line.split(';').collect();

    for segment in segments {
        let parts: Vec<&str> = segment.trim().split_whitespace().collect();
        if parts.len() >= 2 {
            if let Ok(number) = parts[0].parse::<i32>() {
                let color = match parts[1] {
                    "red" => Color::Red,
                    "green" => Color::Green,
                    "blue" => Color::Blue,
                    _ => continue, // Ignore unrecognized colors
                };
                result.insert(color, number);
            }
        }
    }

    result
}
fn main() {
    let input = include_str!("part1.txt");
    let mut id_sum = 0;
    for line in input.lines() {
        let mut red_counter = RED;
        let mut green_counter = GREEN;
        let mut blue_counter = BLUE;


        let parts: Vec<&str> = line.split(":").collect();
        let game_number: u32 = parts[0].trim().trim_start_matches("Game").trim().parse().expect("Failed to parse game number");

        println!("{}", game_number);
        let parsed_line = parse_line(parts[1]);
        for (color, number) in parsed_line {
            println!("{}", number);
            if color == Color::Blue {
                blue_counter -= number
            }
            if color == Color::Red {
                red_counter -= number
            }
            if color == Color::Green {
                green_counter -= number
            }
        }
        println!("{} {} {}", red_counter, blue_counter, green_counter);
        if red_counter >= 0 && blue_counter >= 0 && green_counter >= 0 {
            id_sum += game_number;
        }

        // red_counter -= parsed_line.get(&Color::Red).unwrap_or(&0);
        // blue_counter -= parsed_line.get(&Color::Blue).unwrap_or(&0);
        // green_counter -= parsed_line.get(&Color::Green).unwrap_or(&0);
        

    }
    println!("{}", id_sum);
}
