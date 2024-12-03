mod day;

pub mod days {
    automod::dir!(pub "src/days/");
    aoc_macros::pub_use_solutions!("src/days/");
}

pub use day::Day;
pub use days::*;
