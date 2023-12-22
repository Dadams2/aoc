fn main() {

    let input = include_str!("input1.txt");
    let mut total = 0;

    let char_array: Vec<Vec<char>> = input.lines().map(|line| line.trim().chars().collect()).collect();

    let int_array: Vec<Vec<i32>> = vec![vec![0; char_array[0].len()]; char_array.len()];


    println!("{total}");
}
