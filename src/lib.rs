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
