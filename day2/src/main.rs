use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process::exit;

fn main() {
    // Task 1
    let contents = read_file();
    let commands = read_commands(&contents);
    let (position, depth) = evaluate_commands1(&commands);
    println!("{}", position * depth);

    // Task 2
    let (position, depth) = evaluate_commands2(&commands);
    println!("{}", position * depth);

}

fn read_file() -> String {
    let path = Path::new("input.txt");
    let mut file = File::open(&path).unwrap();
    let mut content = String::new();

    file.read_to_string(&mut content).unwrap();

    return content;
}

fn read_commands(input: &str) -> Vec<Command> {
    let mut commands = Vec::new();

    for line in input.split("\r\n") {
        let command = Command::new_from_line(line);
        commands.push(command);
    }

    return commands;
}

fn evaluate_commands1(commands: &Vec<Command>) -> (i32, i32) {
    let mut position = 0;
    let mut depth = 0;

    for command in commands {
        match command.direction {
            Direction::Forward => position += command.value,
            Direction::Up => depth -= command.value,
            Direction::Down => depth += command.value,
        }
    }

    return (position, depth);
}

fn evaluate_commands2(commands: &Vec<Command>) -> (i32, i32) {
    let mut aim = 0;
    let mut depth = 0;
    let mut position = 0;

    for command in commands {
        match command.direction {
            Direction::Forward => {
                position += command.value;
                depth += aim * command.value;
            },
            Direction::Up => aim -= command.value,
            Direction::Down => aim += command.value,
        }
    }

    return (position, depth);
}

#[derive(Debug)]
enum Direction {
    Down,
    Up,
    Forward,
}

#[derive(Debug)]
struct Command {
    direction: Direction,
    value: i32,
}

impl Command {
    fn new_from_line(line: &str) -> Command {
        let (direction, value) = line.split_once(" ").unwrap();
        let direction = match direction {
            "down" => Direction::Down,
            "up" => Direction::Up,
            "forward" => Direction::Forward,
            _ => exit(1),
        };
        let value= value.parse::<i32>().unwrap();

        return Command{
            direction,
            value,
        };
    }
}

#[cfg(test)]
const TEST_INPUT: &str = "forward 5\r\ndown 5\r\nforward 8\r\nup 3\r\ndown 8\r\nforward 2";

#[cfg(test)]
mod tests_star1 {
    use super::*;

    #[test]
    fn test1() {
        let commands = read_commands(TEST_INPUT);
        let (position, depth) = evaluate_commands1(&commands);

        println!("pos: {}, depth: {}", position, depth);
        assert_eq!(position, 15);
        assert_eq!(depth, 10);
    }
}

#[cfg(test)]
mod tests_star2 {
    use super::*;

    #[test]
    fn test2() {
        let commands = read_commands(TEST_INPUT);
        let (position, depth) = evaluate_commands2(&commands);

        println!("pos: {}, depth: {}", position, depth);
        assert_eq!(position, 15);
        assert_eq!(depth, 60);
    }
}