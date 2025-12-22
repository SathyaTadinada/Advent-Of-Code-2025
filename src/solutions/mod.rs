mod day;

use crate::DayRunner;

macro_rules! days {
    ( $( $num:literal => $module:ident ),* $(,)? ) => {
        $(
            pub mod $module;
        )*

        pub const REGISTRY: &[DayRunner] = &[
            $(
                DayRunner { day: $num, run_fn: $module::run },
            )*
        ];
    }
}

days! {
    1 => day1,
    2 => day2,
    3 => day3,
}
