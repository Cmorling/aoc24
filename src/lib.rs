mod day;

pub mod days {
    automod::dir!(pub "src/days/");

    pub use self::d1::D1p1;
    pub use self::d1::D1p2;
    pub use self::d2::D2p1;
    pub use self::d2::D2p2;
    pub use self::d3::D3p1;
    pub use self::d3::D3p2;
}

pub use day::Day;
pub use days::*;
