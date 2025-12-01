use crate::Solution;

pub fn solve(input: &str) -> Solution {
    Solution {
        part1: part1(input).to_string(),
        part2: part2(input).to_string(),
    }
}

fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(|s| {
            let (dir, num) = s.split_at(1);
            let n: i64 = num.parse().unwrap();
            if dir == "L" { -n } else { n }
        })
        .fold((50, 0), |(cur_pos, zero_count), delta| {
            let new_pos = (cur_pos + delta).rem_euclid(100);
            let zero_count = zero_count + (new_pos == 0) as i64;
            (new_pos, zero_count)
        })
        .1
}

fn part2(input: &str) -> i64 {
    input
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(|s| {
            let (sign, num) = s.split_at(1);
            let n: i64 = num.parse().unwrap();
            if sign == "L" { -n } else { n }
        })
        .fold((50, 0), |(acc, count), x| {
            let mut count = count + (x.abs() / 100);
            let nx = x % 100;
            if (nx > 0 && acc + nx >= 100) || (nx < 0 && nx.abs() >= acc && acc != 0) {
                count += 1
            }
            ((acc + nx).rem_euclid(100), count)
        })
        .1
}

#[cfg(test)]
mod tests {
    use crate::{example, input_test};

    const TEST_INPUT: &str = "

L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

    example!(test_part1, part1, TEST_INPUT, 3);
    example!(test_part2, part2, TEST_INPUT, 6);

    input_test!("01", 1021, 5933);
}
