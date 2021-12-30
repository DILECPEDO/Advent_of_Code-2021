use std::{
    fs::File,
    io::{self, BufRead},
};

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

struct Board {
    cells: Vec<Vec<Cell>>,
}

impl Board {
    fn new() -> Board {
        Board {
            cells: vec![vec![]],
        }
    }
}

impl Board {
    fn from_string(vec: &Vec<String>) -> Board {
        let data: Vec<Vec<Cell>> = vec
            .iter()
            .map(|c| {
                c.split(',')
                    .map(|s| Cell::from(s.parse::<i64>().unwrap()))
                    .collect::<Vec<Cell>>()
            })
            .collect();

        todo!();
    }
}

struct Cell {
    number: i64,
    marked: bool,
}

impl Cell {
    fn from(i: i64) -> Cell {
        Cell {
            number: i,
            marked: false,
        }
    }
}
fn get_data_from_file(input_file: &str) -> (Vec<i64>, Vec<Board>) {
    let mut lines = read_lines(input_file).unwrap();
    let numbers = lines.next().unwrap().unwrap();
    let numbers = numbers
        .split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let lines: Vec<String> = lines
        .into_iter()
        .skip(1)
        .map(|s| s.unwrap().replace("  ", " ").trim().to_string())
        .filter(|line| !(line == ""))
        .collect();
    let board_bodies: Vec<Vec<String>> = lines.chunks(5).map(|s| s.into()).collect();
    let boards = board_bodies
        .iter()
        .map(|v| Board::from_string(v))
        .collect::<Vec<Board>>();

    (numbers, boards)
}
fn main() {
    get_data_from_file("input.txt");
    println!("Hello, world!");
}
