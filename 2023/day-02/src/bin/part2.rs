use std::collections::HashMap;


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
        let mut red_counter = 0;
        let mut green_counter = 0;
        let mut blue_counter = 0;


        let parts: Vec<&str> = line.split(":").collect();
        let game_number: u32 = parts[0].trim().trim_start_matches("Game").trim().parse().expect("Failed to parse game number");

        println!("{}", game_number);
        let segments: Vec<&str> = parts[1].trim().split(';').collect();
        for segment in segments {
            let parsed_line = parse_line(segment);
            for (color, number) in parsed_line {
                if color == Color::Blue {
                    if number > blue_counter {
                        blue_counter = number;
                    }
                }
                if color == Color::Red {
                    if number > red_counter {
                        red_counter = number;
                    }
                }
                if color == Color::Green {
                    if number > green_counter {
                        green_counter = number
                    }
                }
            }
        }
        id_sum += red_counter*blue_counter*green_counter;
    }
    println!("{}", id_sum);
}
