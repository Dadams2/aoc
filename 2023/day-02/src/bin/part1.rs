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

fn parse_line(game: &str) -> HashMap<Color, i32> {
    let mut result = HashMap::new();

    let segments: Vec<&str> = game.split(',').collect();

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

        let parts: Vec<&str> = line.split(":").collect();
        let game_number: u32 = parts[0].trim().trim_start_matches("Game").trim().parse().expect("Failed to parse game number");

        println!("{}", game_number);
        let mut good = true;
        let segments: Vec<&str> = parts[1].trim().split(';').collect();
        for segment in segments {
            let parsed_line = parse_line(segment);
            for (color, number) in parsed_line {
                if color == Color::Blue {
                    if number > BLUE {
                        good = false;
                    }
                }
                if color == Color::Red {
                    if number > RED {
                        good = false;
                    }
                }
                if color == Color::Green {
                    if number > RED {
                        good = false;
                    }
                }
            }
        }
            if good {
                id_sum += game_number;
            }
    }
    println!("{}", id_sum);
}
