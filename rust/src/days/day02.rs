use std::cmp;
use std::collections::HashSet;

use crate::Solution;

pub fn solve(input: &str) -> Solution {
    Solution {
        part1: part1(input).to_string(),
        part2: part2(input).to_string(),
    }
}

#[inline]
fn build_repeated_pat(half: i64, half_digits: u32) -> i64 {
    half * 10i64.pow(half_digits) + half
}

fn count_repeated_patterns(start: i64, end: i64) -> i64 {
    let mut total = 0;

    for digits in (2..=18).step_by(2) {
        let half_digits = digits / 2;
        // Generate all possible half values (10-99 for 4 digits)
        let min_half = 10i64.pow(half_digits - 1);
        let max_half = 10i64.pow(half_digits) - 1;

        let min_pat = build_repeated_pat(min_half, half_digits);

        if min_pat > end {
            break;
        }

        for half in min_half..=max_half {
            let full = build_repeated_pat(half, half_digits);

            if full > end {
                break;
            }

            if full >= start {
                total += full;
            }
        }
    }

    total
}

fn part1(input: &str) -> i64 {
    let mut total = 0;

    for range in input.split(',') {
        let mut parts = range.split('-');
        if let (Some(start_str), Some(end_str)) = (parts.next(), parts.next()) {
            if let (Ok(start), Ok(end)) = (start_str.parse(), end_str.parse()) {
                total += count_repeated_patterns(start, end);
            }
        }
    }

    total
}

fn count_repeated_sequences(start: i64, end: i64) -> i64 {
    let mut seen = HashSet::new();

    for len in 1..=9 {
        let min_pattern = 10i64.pow(len - 1);
        let max_pattern = 10i64.pow(len) - 1;
        let block = 10i64.pow(len);

        let mut mult: i64 = 1;

        for _ in 2..=(19 / len) {
            match mult.checked_mul(block) {
                Some(m) => mult = m + 1,
                None => break,
            }

            let min_val = match min_pattern.checked_mul(mult) {
                Some(v) => v,
                None => break,
            };

            if min_val > end {
                break;
            }

            let heuristic_start = start / mult;
            let actual_start_pat = cmp::max(min_pattern, heuristic_start);

            for pattern in actual_start_pat..=max_pattern {
                let full = match pattern.checked_mul(mult) {
                    Some(v) => v,
                    None => break,
                };

                if full > end {
                    break;
                }

                if full >= start {
                    seen.insert(full);
                }
            }
        }
    }

    seen.iter().sum()
}

fn part2(input: &str) -> i64 {
    let mut total = 0;

    for range in input.split(',') {
        let trimmed = range.trim();
        if trimmed.is_empty() {
            continue;
        }

        let mut parts = trimmed.split('-');
        if let (Some(start_str), Some(end_str)) = (parts.next(), parts.next()) {
            if let (Ok(start), Ok(end)) = (start_str.parse::<i64>(), end_str.parse::<i64>()) {
                total += count_repeated_sequences(start, end);
            }
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use crate::{example, input_test};

    example!(
        test_part1,
        part1,
        "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124",
        1227775554
    );
    example!(
        test_part2,
        part2,
        "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124",
        4174379265
    );

    input_test!("02", 44487518055, 53481866137);
}
