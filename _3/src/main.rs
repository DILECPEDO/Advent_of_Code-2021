use std::{
    fs::File,
    io::{self, BufRead},
};

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_numbers_from_file(input_file: &str) -> Vec<String> {
    let lines = read_lines(input_file).unwrap();
    lines.into_iter().map(|line| line.unwrap()).collect()
}

fn main() {
    let numbers_string = get_numbers_from_file("input.txt");
    let mut gamma_rate = String::from("");
    let mut epsilon_rate = String::from("");
    for i in 0..numbers_string.first().unwrap().len() {
        let digits = numbers_string
            .iter()
            .map(|s| s.chars().nth(i).unwrap())
            .collect::<Vec<char>>();
        let _0s = digits
            .iter()
            .filter(|c| **c == '0' as char)
            .collect::<Vec<&char>>()
            .len();
        let _1s = digits
            .iter()
            .filter(|c| **c == '1' as char)
            .collect::<Vec<&char>>()
            .len();
        assert_eq!(numbers_string.len(), _0s + _1s);
        if _1s > _0s {
            gamma_rate = format!("{}{}", gamma_rate, "1");
            epsilon_rate = format!("{}{}", epsilon_rate, "0");
        } else {
            gamma_rate = format!("{}{}", gamma_rate, "0");
            epsilon_rate = format!("{}{}", epsilon_rate, "1");
        }
    }
    let gamma_rate = i32::from_str_radix(&gamma_rate, 2).expect("Not a binary number!");
    let epsilon_rate = i32::from_str_radix(&epsilon_rate, 2).expect("Not a binary number!");
    let result = gamma_rate * epsilon_rate;
    println!("{}", result);
}
