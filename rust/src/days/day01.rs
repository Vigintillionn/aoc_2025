use crate::Solution;

pub fn solve(input: &str) -> Solution {
    Solution {
        part1: part1(input).to_string(),
        part2: part2(input).to_string(),
    }
}

fn part1(input: &str) -> i64 {
    let mut cur_pos: i64 = 50;
    let mut zero_count: i64 = 0;

    for line in input.lines().map(str::trim).filter(|l| !l.is_empty()) {
        let (dir, _) = line.split_once(' ').unwrap_or_else(|| line.split_at(1));
        let amount: i64 = line[1..].parse().expect("Invalid number");

        match dir {
            "R" => cur_pos = (cur_pos + amount).rem_euclid(100),
            "L" => cur_pos = (cur_pos - amount).rem_euclid(100),
            _ => panic!("Unknown direction {}", dir),
        }

        if cur_pos == 0 {
            zero_count += 1;
        }
    }

    zero_count
}

fn part2(input: &str) -> i64 {
    let mut cur_pos: i64 = 50;
    let mut total_zeros: i64 = 0;

    fn floor_div_100(n: i64) -> i64 {
        if n >= 0 { n / 100 } else { (n - 99) / 100 }
    }

    for line in input.lines().map(str::trim).filter(|l| !l.is_empty()) {
        let (dir, _) = line.split_once(' ').unwrap_or_else(|| line.split_at(1));
        let amount: i64 = line[1..].parse().expect("Invalid number");

        match dir {
            "R" => {
                total_zeros += (cur_pos + amount) / 100;
                cur_pos = (cur_pos + amount) % 100;
            }
            "L" => {
                let upper_bound = cur_pos;
                let lower_bound = cur_pos - amount;

                let count = floor_div_100(upper_bound - 1) - floor_div_100(lower_bound - 1);
                total_zeros += count;

                cur_pos = (cur_pos - amount).rem_euclid(100);
            }
            _ => panic!("Unknown direction {}", dir),
        }
    }

    total_zeros
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
