use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    // Task 1
    let input = read_file();
    let numbers = parse_file(&input);
    let result = analyze_numbers(&numbers);
    println!("{}", result);


    // Task 2
    let windows = get_three_number_windows(&numbers);
    let result = analyze_numbers(&windows);
    println!("{}", result);
}

fn read_file() -> String {
    let path = Path::new("input.txt");
    let mut file = File::open(&path).unwrap();
    let mut content = String::new();

    file.read_to_string(&mut content).unwrap();

    return content;
}

fn parse_file(content: &str) -> Vec<i32> {
    let mut values = Vec::new();

    for split in content.split("\r\n") {
        values.push(split.parse::<i32>().unwrap())
    }

    return values;
}

fn analyze_numbers(numbers: &Vec<i32>) -> i32{
    let count = numbers.iter().skip(1).enumerate().map(|(index, value)| {
        return if *value > numbers[index] {
            1
        } else {
            0
        }
    }).sum();

    return count;
}

fn get_three_number_windows(numbers: &Vec<i32>) -> Vec<i32> {
    let windows = numbers.iter().skip(2).enumerate().map(|(index, _)| {
        return numbers[index] + numbers[index + 1] + numbers[index + 2];
    }).collect();

    return windows;
}

#[cfg(test)]
const TEST_INPUT: &str = "199\r\n200\r\n208\r\n210\r\n200\r\n207\r\n240\r\n269\r\n260\r\n263";

#[cfg(test)]
mod tests_star1 {
    use super::*;

    #[test]
    fn test1() {
        let vec = parse_file(TEST_INPUT);
        let result = analyze_numbers(&vec);

        assert_eq!(result, 7);
    }
}

#[cfg(test)]
mod tests_star2 {
    use super::*;

    #[test]
    fn get_three_number_windows_should_return_correct_sequence() {
        // Arrange
        let vec = parse_file(TEST_INPUT);

        // Act
        let windows = get_three_number_windows(&vec);

        assert_eq!(windows.len(), 8);
        assert_eq!(windows, vec![607, 618, 618, 617, 647, 716, 769, 792]);
    }

    #[test]
    fn test2() {
        let vec = parse_file(TEST_INPUT);
        let windows = get_three_number_windows(&vec);
        let result = analyze_numbers(&windows);

        assert_eq!(result, 5);
    }
}