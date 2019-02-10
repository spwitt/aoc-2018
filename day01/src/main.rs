use std::collections::HashSet;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let result1 = part1(&input);
    println!("Part 1 result is {}", result1);
    let result2 = part2(&input);
    println!("Part 2 result is {}", result2);
}

fn part1(input: &str) -> i32 {
    let mut total: i32 = 0;
    for line in input.lines() {
        if let Ok(val) = line.parse::<i32>() {
            total += val;
        }
    }
    total
}

fn part2(input: &str) -> i32 {
    let mut seen_freqs = HashSet::new();
    let mut total: i32 = 0;
    seen_freqs.insert(total);
    loop {
        for line in input.lines() {
            if let Ok(val) = line.parse::<i32>() {
                total += val;
                if !seen_freqs.insert(total) {
                    return total;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1_1() {
        let result = part1("+1\n+1\n+1");
        assert_eq!(result, 3);
    }

    #[test]
    fn sample1_2() {
        let result = part1("+1\n+1\n-2");
        assert_eq!(result, 0);
    }

    #[test]
    fn sample1_3() {
        let result = part1("-1\n-2\n-3");
        assert_eq!(result, -6);
    }

    #[test]
    fn sample1_4() {
        let result = part1("+1\n-2\n+3\n+1");
        assert_eq!(result, 3);
    }

    #[test]
    fn sample2_1() {
        let result = part2("+1\n-1");
        assert_eq!(result, 0);
    }

    #[test]
    fn sample2_2() {
        let result = part2("+3\n+3\n+4\n-2\n-4");
        assert_eq!(result, 10);
    }

    #[test]
    fn sample2_3() {
        let result = part2("-6\n+3\n+8\n+5\n-6\n");
        assert_eq!(result, 5);
    }

    #[test]
    fn sample2_4() {
        let result = part2("+7\n+7\n-2\n-7\n-4\n");
        assert_eq!(result, 14);
    }
}
