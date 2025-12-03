use crate::Solution;

pub fn solve(input: &str) -> Solution {
    Solution {
        part1: part1(input).to_string(),
        part2: part2(input).to_string(),
    }
}

fn part1(input: &str) -> i64 {
    let mut total = 0;
    for line in input.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let digits: Vec<i64> = trimmed
            .chars()
            .filter_map(|c| c.to_digit(10))
            .map(|d| d as i64)
            .collect();
        if digits.len() < 2 {
            continue;
        }

        let mut suffix_max = vec![0; digits.len()];
        suffix_max[digits.len() - 1] = digits[digits.len() - 1];
        for i in (0..digits.len() - 1).rev() {
            suffix_max[i] = suffix_max[i + 1].max(digits[i]);
        }

        let mut max = 0;
        for i in 0..digits.len() - 1 {
            let joltage = digits[i] * 10 + suffix_max[i + 1];
            max = max.max(joltage);
        }

        total += max;
    }
    total
}

fn part2(input: &str) -> i64 {
    let mut total: i64 = 0;
    const TARGET: usize = 12;
    for line in input.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let digits: Vec<i64> = trimmed
            .chars()
            .filter_map(|c| c.to_digit(10))
            .map(|d| d as i64)
            .collect();
        if digits.len() < TARGET {
            continue;
        }
        let mut drops = digits.len() - TARGET;
        let mut stack = Vec::with_capacity(TARGET);
        for &digit in &digits {
            while drops > 0 && !stack.is_empty() && *stack.last().unwrap() < digit {
                stack.pop();
                drops -= 1;
            }
            stack.push(digit);
        }
        stack.truncate(TARGET);
        let blank = stack.iter().fold(0, |acc, &d| acc * 10 + d);
        total += blank;
    }
    total
}

#[cfg(test)]
mod tests {
    use crate::{example, input_test};

    example!(
        test_part1,
        part1,
        "987654321111111
811111111111119
234234234234278
818181911112111",
        357
    );
    example!(
        test_part2,
        part2,
        "987654321111111
811111111111119
234234234234278
818181911112111",
        3121910778619
    );

    input_test!("03", 17207, 170997883706617);
}
