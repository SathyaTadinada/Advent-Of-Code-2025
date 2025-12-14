#[macro_export]
macro_rules! day {
    ($title:literal => { $p1:ident, $p2:ident }) => {
        pub const TITLE: &str = $title;

        pub fn run(input: &str, which: $crate::Part) {
            let day_no = $crate::extract_day_number(file!());
            println!("--- Day {}: {} ---", day_no, TITLE);

            match which {
                $crate::Part::One => {
                    let result = $p1(input);
                    println!("Part 1: {:#?}", result);
                }
                $crate::Part::Two => {
                    let result = $p2(input);
                    println!("Part 2: {:#?}", result);
                }
                $crate::Part::Both => {
                    let r1 = $p1(input);
                    println!("Part 1: {:#?}", r1);
                    let r2 = $p2(input);
                    println!("Part 2: {:#?}", r2);
                }
            }
        }
    };

    ($title:literal => { $p1:ident }) => {
        pub const TITLE: &str = $title;

        pub fn run(input: &str, which: $crate::Part) {
            let day_no = $crate::extract_day_number(file!());
            println!("--- Day {}: {} ---", day_no, TITLE);

            match which {
                $crate::Part::One | $crate::Part::Both => {
                    let result = $p1(input);
                    println!("Part 1: {:#?}", result);
                }
                $crate::Part::Two => {
                    eprintln!("Part 2 not implemented for Day {}", day_no);
                }
            }
        }
    };
}

#[macro_export]
macro_rules! test_day {
    ($test_input:literal, {}) => {};

    ($test_input:literal, { $p1:ident => $p1_answer:expr $(,)? }) => {
        #[cfg(test)]
        mod test {
            use super::*;
            const TEST: &str = $test_input;

            #[test]
            fn p1_test() {
                assert_eq!($p1(TEST), $p1_answer);
            }
        }
    };

    ($test_input:literal, { $p1:ident => $p1_answer:expr, $p2:ident => $p2_answer:expr $(,)? }) => {
        #[cfg(test)]
        mod test {
            use super::*;
            const TEST: &str = $test_input;

            #[test]
            fn p1_test() {
                assert_eq!($p1(TEST), $p1_answer);
            }

            #[test]
            fn p2_test() {
                assert_eq!($p2(TEST), $p2_answer);
            }
        }
    };
}
