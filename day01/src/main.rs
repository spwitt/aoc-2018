use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let result1 = part1(input);
    println!("Part 1 result is {}", result1);
}

fn part1(input: String) -> i32 {
    let lines = input.lines();
    let mut total: i32 = 0;
    for line in lines {
        if let Ok(val) = line.parse::<i32>() {
            total += val;
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let input = "+1\n+1\n+1".to_string();
        let result = part1(input);
        assert_eq!(result, 3);
    }

    #[test]
    fn sample2() {
        let input = "+1\n+1\n-2".to_string();
        let result = part1(input);
        assert_eq!(result, 0);
    }

    #[test]
    fn sample3() {
        let input = "-1\n-2\n-3".to_string();
        let result = part1(input);
        assert_eq!(result, -6);
    }

    #[test]
    fn sample4() {
        let input = "+1\n-2\n+3\n+1".to_string();
        let result = part1(input);
        assert_eq!(result, 3);
    }
}
