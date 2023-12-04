
fn main() {
    let input = include_str!("input1.txt");
    let mut total = 0;

    for val in input.lines() {
        let digit_values: Vec<u32> = val.chars().filter_map(|c| c.to_digit(10)).collect();
        total += 10*digit_values.first().unwrap_or(&0) + digit_values.last().unwrap_or(&0);
    }
    println!("{total}");
}
