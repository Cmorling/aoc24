use aoc_macros::DayEnum;
mod day;
mod util;

pub mod days {
    automod::dir!(pub "src/days");

    // Have to type these out for linting services
    pub use self::d1::*;
    pub use self::d2::*;
    pub use self::d3::*;
    pub use self::d4::*;
    pub use self::d5::*;
    pub use self::d6::*;
    pub use self::d7::*;
    pub use self::d8::*;
}

#[derive(DayEnum)]

pub enum DayEnum {
    D1(days::D1),
    D2(days::D2),
    D3(days::D3),
    D4(days::D4),
    D5(days::D5),
    D6(days::D6),
    D7(days::D7),
    D8(days::D8),
}

pub use day::Day;
pub use days::*;
pub use util::*;
