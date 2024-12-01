use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(path) = args.get(1) {
        let (left, right): (Vec<_>, Vec<_>) = BufReader::new(File::open(path)?)
            .lines()
            .map_while(Result::ok)
            .filter_map(|line| {
                if let Ok([left, right]) = line
                    .split_whitespace()
                    .map(|s| s.parse::<u32>())
                    .collect::<Result<Vec<_>, _>>()
                    .as_deref()
                {
                    Some((*left, *right))
                } else {
                    None
                }
            })
            .unzip();

        println!(
            "Total distance: {}",
            total_distance(left.clone(), right.clone())
        );
        println!("Similarity score: {}", similarity_score(&left, &right));

        Ok(())
    } else {
        Err("Usage: day01 INPUT_FILE_PATH".into())
    }
}

fn total_distance(mut left: Vec<u32>, mut right: Vec<u32>) -> u32 {
    left.sort();
    right.sort();

    left.iter()
        .zip(right.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .sum()
}

fn similarity_score(left: &[u32], right: &[u32]) -> u64 {
    let mut occurrences = HashMap::new();

    right.iter().for_each(|&n| {
        *occurrences.entry(n).or_insert(0u64) += 1;
    });

    left.iter()
        .map(|n| *n as u64 * occurrences.get(n).unwrap_or(&0))
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_total_distance() {
        let left = vec![3, 4, 2, 1, 3, 3];
        let right = vec![4, 3, 5, 3, 9, 3];

        assert_eq!(11, total_distance(left, right));
    }

    #[test]
    fn test_similarity_score() {
        let left = vec![3, 4, 2, 1, 3, 3];
        let right = vec![4, 3, 5, 3, 9, 3];

        assert_eq!(31, similarity_score(&left, &right));
    }
}
