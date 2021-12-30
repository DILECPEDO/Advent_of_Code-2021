use std::{
    fs::File,
    io::{self, BufRead},
};

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_commands_from_file(input_file: &str) -> Vec<Command> {
    let lines = read_lines(input_file).unwrap();
    lines
        .into_iter()
        .map(|line| Command::from_string(&line.unwrap()))
        .collect()
}

struct Submarine {
    coordinate: Position,
    aim: i64,
}

impl Submarine {
    fn new() -> Submarine {
        Submarine {
            coordinate: Position::new(),
            aim: 0,
        }
    }
    fn go_v1(&mut self, commands: &Command) {
        use crate::Command::*;
        match commands {
            Up(y) => self.coordinate.y -= y,
            Down(y) => self.coordinate.y += y,
            Forward(x) => self.coordinate.x += x,
        }
    }
    fn go_v2(&mut self, commands: &Command) {
        use crate::Command::*;
        match commands {
            Up(y) => self.aim -= y,
            Down(y) => self.aim += y,
            Forward(x) => {
                self.coordinate.x += x;
                self.coordinate.y += self.aim * x;
            }
        }
    }
}

#[derive(Debug)]
struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn new() -> Position {
        Position { x: 0, y: 0 }
    }
}
#[derive(Debug)]
enum Command {
    Up(i64),
    Down(i64),
    Forward(i64),
}

impl Command {
    fn from_string(string: &str) -> Command {
        use crate::Command::*;
        let parts = string.split(" ").collect::<Vec<&str>>();
        let direction = parts.first().unwrap().to_owned();
        let value = parts.last().unwrap().parse::<i64>().unwrap();
        match direction {
            "forward" => Forward(value),
            "up" => Up(value),
            "down" => Down(value),
            err => panic!("Invalid direction in file: {}", err),
        }
    }
}

fn main() {
    let input_file = "input.txt";
    let commands = get_commands_from_file(input_file);

    // PART ONE
    let mut sub = Submarine::new();
    commands.iter().for_each(|comm| sub.go_v1(comm));

    let result = sub.coordinate.x * (sub.coordinate.y).max(0);
    println!("Final result: {}", result);

    // PART TWO
    let mut sub = Submarine::new();
    commands.iter().for_each(|comm| sub.go_v2(comm));

    let result = sub.coordinate.x * (sub.coordinate.y).max(0);
    println!("Final result using aim: {}", result);
}
