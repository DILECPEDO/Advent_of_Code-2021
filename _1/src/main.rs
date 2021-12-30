use std::{
    fs::File,
    io::{self, BufRead},
};

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_numbers_from_file(input_file: &str) -> Vec<i64> {
    let lines = read_lines(input_file).unwrap();
    lines
        .into_iter()
        .map(|line| line.unwrap().parse::<i64>().unwrap())
        .collect()
}

fn increases_in_vec<T: Ord>(vec: &Vec<T>) -> usize {
    vec.iter()
        .zip(vec.iter().skip(1))
        .map(|(a, b)| a < b)
        .filter(|i| *i)
        .collect::<Vec<bool>>()
        .len()
}
fn main() {
    let input_file = "input.txt";
    let numbers = get_numbers_from_file(input_file);

    // PART ONE
    let increases = increases_in_vec(&numbers);

    println!("Increase: {}", increases);

    // PART TWO
    let mut sums: Vec<i64> = vec![];
    for i in 2..numbers.len() {
        let sum = numbers[i - 2] + numbers[i - 1] + numbers[i];
        sums.push(sum);
    }
    let sliding_window_increases = increases_in_vec(&sums);

    println!(
        "Increase using three-measurement sliding window: {}",
        sliding_window_increases
    );
}
