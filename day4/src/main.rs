use std::fs::File;
use std::io::Read;
use std::path::Path;
use colored::Colorize;

const TEST_INPUT: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

fn main() {
    let content = read_file();
    let draws = read_draw_numbers(&content);
    let mut boards = read_boards(&content);
    let score = find_last_winning_board(&mut boards, &draws);

    println!("{}", score);
}

fn read_file() -> String {
    let path = Path::new("input.txt");
    let mut file = File::open(&path).unwrap();
    let mut content = String::new();

    file.read_to_string(&mut content).unwrap();

    return content;
}

fn read_draw_numbers(input: &str) -> Vec<i32> {
    let (draw_numbers, _) = input.split_once("\n\n").unwrap();
    let draw_numbers = draw_numbers.split(",");

    let draw_numbers = draw_numbers.map(|num| num.parse::<i32>().unwrap() ).collect();
    return draw_numbers;
}

fn read_boards(input: &str) -> Vec<Board> {
    let (_, boards) = input.split_once("\n\n").unwrap();
    let boards: Vec<&str> = boards.split("\n\n").collect();
    let boards = boards.iter().map(|board| return Board::new(board)).collect();

    return boards;
}

fn play(boards: &mut Vec<Board>, draws: &Vec<i32>) -> i32 {
    for draw in draws {

        for board in &mut *boards {
            let win = board.check_number(*draw);

            if win {
                return board.calculate_score(*draw);
            }
        }

    }

    return 0;
}

fn find_last_winning_board(boards: &mut Vec<Board>, draws: &Vec<i32>) -> i32{
    for i in 0..draws.len() {
        let draw = draws[i];
        let mut j = 0;

        while j < boards.len() {
            let win = boards[j].check_number(draw);

            if win {
                if boards.len() == 1 {
                    return boards[j].calculate_score(draw);
                }

                 boards.remove(j);
            } else {
                j += 1;
            }
        }
    }

    return 0;
}

struct Board {
    number_matrix: Vec<Vec<i32>>,
    bitvector: Vec<Vec<bool>>,
}

impl Board {
    fn new(board_str: &str) -> Board {
        let board = board_str.split("\n");
        let mut matrix: Vec<Vec<i32>> = Vec::new();

        for line in board {
            let matrix_line = line
                .split_whitespace()
                .map(|num|
                    num.parse::<i32>().unwrap())
                .collect();
            matrix.push(matrix_line);
        }

        let len = matrix[0].len();

        return Board{
            number_matrix: matrix,
            bitvector: vec![vec![false; len]; len],
        }
    }

    fn calculate_score(&self, last_number: i32) -> i32{
        let mut sum = 0;

        for i in 0..self.bitvector.len() {
            for j in 0..self.bitvector[0].len() {
                if !self.bitvector[i][j] {
                    sum += self.number_matrix[i][j];
                }
            }
        }

        return sum * last_number;
    }

    fn check_number(&mut self, number: i32) -> bool {
        for i in 0..self.number_matrix.len() {
            for j in 0..self.number_matrix[i].len() {
                if self.number_matrix[i][j] == number {
                    self.bitvector[i][j] = true;
                }
            }
        }

        for i in 0..self.number_matrix.len() {
            for j in 0..self.number_matrix[i].len() {
                if self.bitvector[i][j] {
                    print!("{0: <5} ", format!("+{}+", self.number_matrix[i][j]).green());
                } else {
                    print!("{0: <5} ", self.number_matrix[i][j])
                }
            }
            println!();
        }
        println!();

        return self.evaluate();
    }

    fn evaluate(&self) -> bool {
        // Check columns
        for i in 0..self.bitvector.len() {
            let mut result_column = true;
            let mut result_row = true;

            for j in 0..self.bitvector[i].len() {
                if self.bitvector[i][j] == false {
                    result_column = false;
                }

                if self.bitvector[j][i] == false {
                    result_row = false;
                }
            }

            if result_column || result_row {
                return true;
            }
        }

        return false;
    }
}

#[cfg(test)]
mod tests_star1 {
    use super::*;

    #[test]
    fn test1() {
        let draws = read_draw_numbers(TEST_INPUT);
        let mut boards = read_boards(TEST_INPUT);
        let score = play(&mut boards, &draws);

        assert_eq!(score, 4512);
    }

    #[test]
    fn test2() {
        let draws = read_draw_numbers(TEST_INPUT);
        let mut boards = read_boards(TEST_INPUT);
        let score = find_last_winning_board(&mut boards, &draws);

        assert_eq!(score, 1924);
    }
}
