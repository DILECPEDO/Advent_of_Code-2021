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

fn get_0_and_1_in_nth_place(numbers_string: &Vec<String>, index: usize) -> (usize, usize) {
    let digits = numbers_string
        .iter()
        .map(|s| s.chars().nth(index).unwrap())
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
    (_0s, _1s)
}

fn bin_string_to_i32(string: &String) -> i32 {
    i32::from_str_radix(&string, 2).expect("Not a binary number!")
}
fn main() {
    let numbers_string = get_numbers_from_file("input.txt");

    // PART ONE
    let mut gamma_rate = String::from("");
    let mut epsilon_rate = String::from("");
    for i in 0..numbers_string.first().unwrap().len() {
        let (_0s, _1s) = get_0_and_1_in_nth_place(&numbers_string, i);
        assert_eq!(numbers_string.len(), _0s + _1s);
        if _1s > _0s {
            gamma_rate = format!("{}{}", gamma_rate, "1");
            epsilon_rate = format!("{}{}", epsilon_rate, "0");
        } else {
            gamma_rate = format!("{}{}", gamma_rate, "0");
            epsilon_rate = format!("{}{}", epsilon_rate, "1");
        }
    }
    let gamma_rate = bin_string_to_i32(&gamma_rate);
    let epsilon_rate = bin_string_to_i32(&epsilon_rate);
    let result = gamma_rate * epsilon_rate;
    println!("power consumption: {}", result);

    // PART TWO
    part_two(numbers_string);
}
enum Element {
    Oxigen,
    CO2,
}

fn part_two(numbers_string: Vec<String>) {
    let ox_rating = find_rating(numbers_string.to_owned(), 0, Element::Oxigen);
    let ox_rating = ox_rating.first().unwrap();
    let co2_rating = find_rating(numbers_string.to_owned(), 0, Element::CO2);
    let co2_rating = co2_rating.first().unwrap();
    let result = bin_string_to_i32(&ox_rating) * bin_string_to_i32(&co2_rating);
    println!("life support rating: {}", result);
}

fn find_rating(numbers_string: Vec<String>, index: usize, element: Element) -> Vec<String> {
    assert!(
        index <= numbers_string.first().unwrap().len(),
        "Index is larger then bynary number"
    );
    if numbers_string.len() > 1 {
        let (_0s, _1s) = get_0_and_1_in_nth_place(&numbers_string, index);
        let common = match element {
            Element::Oxigen => {
                if _0s > _1s {
                    "0"
                } else {
                    "1"
                }
            }
            Element::CO2 => {
                if _0s <= _1s {
                    "0"
                } else {
                    "1"
                }
            }
        };
        let numbers_string = numbers_string
            .into_iter()
            .filter(|s| s.chars().nth(index).unwrap().to_string() == common)
            .collect::<Vec<String>>();
        return find_rating(numbers_string, index + 1, element);
    } else {
        numbers_string
    }
}
