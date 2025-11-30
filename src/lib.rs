pub mod days;

pub struct Solution {
    pub part1: String,
    pub part2: String,
}

pub trait Solver {
    fn solve(input: &str) -> Solution;
}

#[macro_export]
macro_rules! example {
    ($name:ident, $func:ident, $input:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let input = $input;
            assert_eq!(super::$func(input), $expected);
        }
    };
}

#[macro_export]
macro_rules! input_test {
    ($day:expr, $p1_ans:expr, $p2_ans:expr) => {
        #[test]
        fn test_real_input() {
            let input = include_str!(concat!("../../inputs/day", $day, ".txt"));
            assert_eq!(super::part1(input), $p1_ans);
            assert_eq!(super::part2(input), $p2_ans);
        }
    };
}
