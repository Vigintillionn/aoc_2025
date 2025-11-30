pub mod day01;
// [IMPORT_MARKER]

pub fn get_solver(day: u8) -> Option<fn(&str) -> crate::Solution> {
    match day {
        1 => Some(day01::solve),
        // [MATCH_MARKER]
        _ => None,
    }
}
