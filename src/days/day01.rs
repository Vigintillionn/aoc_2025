use crate::Solution;

pub fn solve(input: &str) -> Solution {
    Solution {
        part1: part1(input).to_string(),
        part2: part2(input).to_string(),
    }
}

fn part1(input: &str) -> i64 {
    0
}

fn part2(input: &str) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use crate::example;

    example!(test_part1, part1, "test input", 0);
    example!(test_part2, part2, "test input", 0);
}
