fn main() {
    println!("Hello, world!");
}

fn split_input(input: &str) -> Vec<&str> {
    return input.split("\r\n").collect();
}

fn get_gamma_rate(numbers: Vec<&str>) -> Vec<i8> {
    // Vector for each bit. If count < 0 more zeroes, count > 0 more ones.
    let mut bit_counts: Vec<i32> = vec![0, numbers[0].len().try_into().unwrap()];
    let mut gamma_rate: Vec<i8> = vec![0, numbers[0].len().try_into().unwrap()];

    for number in numbers {
        let chars = number.chars().enumerate().map(|(index, char)| {
            if char == '1' {
                bit_counts[index] += 1;
            } else {
                bit_counts[index] -= 1;
            }
        });
    }

    for i in 0..bit_counts.len() {
        gamma_rate[i] = if bit_counts[i] > 0 {
            0
        } else {
            1
        };
    }

    return gamma_rate;
}

#[cfg(test)]
const TEST_INPUT: &str = "00100\r\n11110\r\n10110\r\n10111\r\n10101\r\n01111\r\n00111\r\n11100\r\n10000\r\n11001\r\n00010\r\n01010";

#[cfg(test)]
mod tests_star1 {
    use super::*;

    #[test]
    fn test1() {
        let numbers = split_input(TEST_INPUT);
        let gamma_rate = get_gamma_rate(numbers);

        assert_eq!(gamma_rate[0], 1);
        assert_eq!(gamma_rate[1], 0);
        assert_eq!(gamma_rate[2], 1);
        assert_eq!(gamma_rate[3], 1);
        assert_eq!(gamma_rate[4], 0);
    }

}