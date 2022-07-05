use std::fmt;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    let input = read_file();

    let lines = read_vent_lines(&input);
    let map = create_vent_map(&lines);
    let number_of_dangerous_points = analyze_vent_map(&map);

    println!("{}", number_of_dangerous_points);
}

fn read_file() -> String {
    let path = Path::new("input.txt");
    let mut file = File::open(&path).unwrap();
    let mut content = String::new();

    file.read_to_string(&mut content).unwrap();

    return content;
}

fn read_vent_lines(input: &str) -> Vec<Line>{
    let lines = input.split("\n").map(|point_pair| {
        let (line_from, line_to) = point_pair.split_once(" -> ").unwrap();
        let line = Line::new(line_from, line_to);
        return line;
    }).collect();

    return lines;
}

fn find_map_size(lines: &Vec<Line>) -> (i32, i32) {
    let mut x_size = 0;
    let mut y_size = 0;

    for line in lines {
        if line.x1 > x_size {
            x_size = line.x1;
        }
        if line.x2 > x_size {
            x_size = line.x2;
        }
        if line.y1 > y_size {
            y_size = line.y1;
        }
        if line.y2 > y_size {
            y_size = line.y2;
        }
    }

    return (x_size + 1, y_size + 1);
}

fn create_vent_map(lines: &Vec<Line>) -> Vec<Vec<i32>> {
    let (s_size, y_size) = find_map_size(lines);
    let mut map = vec![vec![0; s_size as usize]; y_size as usize];

    for line in lines {
        if line.x1 == line.x2 {
            let start = if line.y1 > line.y2 { line.y2 as usize } else { line.y1 as usize };
            let end = if line.y1 > line.y2 { line.y1 as usize } else { line.y2 as usize };

            for i in start..(end + 1) {
                map[i][line.x1 as usize] += 1;
            }
        }
        else if line.y1 == line.y2 {
            let start = if line.x1 > line.x2 { line.x2 as usize } else { line.x1 as usize };
            let end = if line.x1 > line.x2 { line.x1 as usize } else { line.x2 as usize };

            for i in start..(end + 1) {
                 map[line.y1 as usize][i] += 1;
            }
        }
        else {
            continue;
        }
    }

    return map;
}

fn analyze_vent_map(map: &Vec<Vec<i32>>) -> i32 {
    let mut result = 0;

    // print_2d_vector(map);

    for line in map {
        for number in line {
            if *number > 1 {
                result += 1;
            }
        }
    }

    return result;
}

fn print_2d_vector(vector: &Vec<Vec<i32>>) {
    for vec in vector {
        for num in vec {
            print!("{} ", *num);
        }

        println!();
    }
}

struct Line {
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32,
}

impl Line {
    fn new(line_from: &str, line_to: &str) -> Self {
        let (x1, y1) = line_from.split_once(",").unwrap();
        let (x2, y2) = line_to.split_once(",").unwrap();

        let x1 = x1.parse::<i32>().unwrap();
        let x2 = x2.parse::<i32>().unwrap();
        let y1 = y1.parse::<i32>().unwrap();
        let y2 = y2.parse::<i32>().unwrap();

        return Line{x1, x2, y1, y2};
    }
}

impl fmt::Debug for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{} -> {},{}", self.x1,self.y1, self.x2, self.y2)
    }
}

#[cfg(test)]
const TEST_INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

#[cfg(test)]
mod tests_star1 {
    use super::*;

    #[test]
    fn test1() {
        let lines = read_vent_lines(TEST_INPUT);
        let map = create_vent_map(&lines);
        let number_of_dangerous_points = analyze_vent_map(&map);

        assert_eq!(number_of_dangerous_points, 5);
    }
}
