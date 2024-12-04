mod day;
mod util;

pub mod days {
    automod::dir!(pub "src/days/");
    aoc_macros::pub_use_solutions!("src/days/");
    // the one under this is for LSP services
    pub use self::d4::*;
}

pub use day::Day;
pub use util::*;
pub use days::*;
