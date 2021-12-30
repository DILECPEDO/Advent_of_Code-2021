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
    fn from_string(vec: Vec<String>) -> Board {
        let data: Vec<Vec<i64>> = vec
            .iter()
            .map(|c| {
                c.split(',')
                    .map(|s| s.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>()
            })
            .collect();

        todo!();
    }
}

struct Cell {
    number: i64,
    marked: bool,
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
    let board_bodys: Vec<Vec<String>> = lines.chunks(5).map(|s| s.into()).collect();
    // let boards = board_bodys.iter().map(|v| )

    println!("{:?}", board_bodys);
    todo!();
}
fn main() {
    get_data_from_file("input.txt");
    println!("Hello, world!");
}
