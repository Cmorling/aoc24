mod day;

pub mod days {
    automod::dir!(pub "src/days/");

    pub use self::d1::D1p1;
    pub use self::d1::D1p2;
}
pub use days::*;
pub use day::Day;
