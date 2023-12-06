

const RED: i32 = 12;
const GREEN: i32 = 13;
const BLUE: i32 = 14;

fn main() {
    let input = include_str!("part1.txt");
    for line in input.lines() {
        let mut red_counter = RED;
        let mut green_counter = GREEN;
        let mut blue_counter = BLUE;


        let mut splitter = line.split(":");
        let id_str = splitter.next();
        let game_str = splitter.next();

        let game_id = id_str.unwrap().chars().find(|c| c.is_digit(10)).and_then(|c| c.to_digit(10));
        let mut game_splitter = game_str.unwrap().split(";");

        

    }
    println!("{}", 0);
}
