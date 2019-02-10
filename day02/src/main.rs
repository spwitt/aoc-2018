use std::collections::HashMap;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let result1 = part1(&input);
    println!("Part 1 result is {}", result1);
    let result2 = part2(&input);
    println!("Part 2 result is {}", result2);
}

fn part1(input: &str) -> usize {
    let matches = count_matches(input.lines().collect(), vec![2, 3]);
    matches.iter().fold(1, |acc, (_, &count)| acc * count)
}

fn get_char_counts(input: &str) -> HashMap<char, usize> {
    let mut char_counts = HashMap::new();
    for c in input.chars() {
        let count = char_counts.entry(c).or_insert(0);
        *count += 1;
    }
    char_counts
}

fn count_matches(input: Vec<&str>, interest: Vec<usize>) -> HashMap<usize, usize> {
    let mut counts = HashMap::new();
    for line in input {
        let char_counts = get_char_counts(line);
        for &val in interest.iter() {
            if char_counts.iter().any(|(_, &count)| count == val) {
                let interest_count = counts.entry(val).or_insert(0);
                *interest_count += 1;
            }
        }
    }
    counts
        .into_iter()
        .filter(|(interest_count, _)| {
            interest
                .iter()
                .any(|interest_val| interest_count == interest_val)
        })
        .collect()
}

fn part2(input: &str) -> String {
    let (id1, id2) = find_box_ids(input.lines().collect(), 1).unwrap();
    extract_same(id1, id2)
}

fn find_box_ids(input: Vec<&str>, diff_count: usize) -> Option<(&str, &str)> {
    for (i, id1) in input.iter().enumerate() {
        for id2 in input.iter().skip(i + 1) {
            if id1
                .as_bytes()
                .iter()
                .zip(id2.as_bytes().iter())
                .filter(|(a, b)| a != b)
                .count()
                == diff_count
            {
                return Some((id1, id2));
            }
        }
    }
    None
}

fn extract_same(id1: &str, id2: &str) -> String {
    String::from_utf8(
        id1.as_bytes()
            .iter()
            .zip(id2.as_bytes().iter())
            .filter(|(a, b)| a == b)
            .map(|(&a, _)| a)
            .collect(),
    )
    .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_vec1() -> Vec<&'static str> {
        vec![
            "abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab",
        ]
    }

    fn sample_vec2() -> Vec<&'static str> {
        vec![
            "abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz",
        ]
    }

    #[test]
    fn test_get_char_counts() {
        let box_ids = sample_vec1();
        let counts = get_char_counts(box_ids[0]);
        assert_eq!(counts.len(), 6);
        assert_eq!(counts.values().sum::<usize>(), 6);
        let counts = get_char_counts(box_ids[1]);
        assert_eq!(counts.len(), 3);
        assert_eq!(counts.values().sum::<usize>(), 6);
        let counts = get_char_counts(box_ids[2]);
        assert_eq!(counts.len(), 5);
        assert_eq!(counts.values().sum::<usize>(), 6);
        let counts = get_char_counts(box_ids[3]);
        assert_eq!(counts.len(), 4);
        assert_eq!(counts.values().sum::<usize>(), 6);
        let counts = get_char_counts(box_ids[4]);
        assert_eq!(counts.len(), 4);
        assert_eq!(counts.values().sum::<usize>(), 6);
        let counts = get_char_counts(box_ids[5]);
        assert_eq!(counts.len(), 5);
        assert_eq!(counts.values().sum::<usize>(), 6);
        let counts = get_char_counts(box_ids[6]);
        assert_eq!(counts.len(), 2);
        assert_eq!(counts.values().sum::<usize>(), 6);
    }

    #[test]
    fn test_count_matches() {
        let box_ids = sample_vec1();
        let matches = count_matches(box_ids, vec![2, 3]);
        assert_eq!(matches.len(), 2);
        assert_eq!(*matches.get(&2).unwrap(), 4);
        assert_eq!(*matches.get(&3).unwrap(), 3);
    }

    #[test]
    fn test_find_box_ids() {
        let box_ids = sample_vec2();
        let (id1, id2) = find_box_ids(box_ids, 1).unwrap();
        assert_eq!(id1, "fghij");
        assert_eq!(id2, "fguij");
    }

    #[test]
    fn test_extract_same() {
        let diff = extract_same("fghij", "fguij");
        assert_eq!(diff, "fgij");
    }
}
