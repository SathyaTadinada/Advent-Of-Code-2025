crate::day!("Gift Shop" => {
    part_1,
    part_2
});

fn part_1(data: &str) -> isize {
    let ranges = data.trim().split(',').collect::<Vec<&str>>();
    let mut total_count = 0;
    for range in ranges {
        let bounds = range.split('-').collect::<Vec<&str>>();
        let start: isize = bounds[0].parse().unwrap();
        let end: isize = bounds[1].parse().unwrap();
        for i in start..=end {
            let s = i.to_string();
            if s[0..s.len() / 2] == s[s.len() / 2..s.len()] {
                total_count += i;
            }
        }
    }
    total_count
}

fn part_2(data: &str) -> isize {
    let ranges = data.trim().split(',').collect::<Vec<&str>>();
    let mut total_count = 0;
    for range in ranges {
        let bounds = range.split('-').collect::<Vec<&str>>();
        let start: isize = bounds[0].parse().unwrap();
        let end: isize = bounds[1].parse().unwrap();
        for i in start..=end {
            if is_invalid_id(i) {
                total_count += i;
            }
        }
    }
    total_count
}

fn is_invalid_id(id: isize) -> bool {
    let b = id.to_string().into_bytes();
    let n = b.len();

    for p in 1..=n / 2 {
        if !n.is_multiple_of(p) {
            continue;
        }
        if (p..n).all(|i| b[i] == b[i % p]) {
            return true;
        }
    }
    false
}

crate::test_day!(

"
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124
",
{
    part_1 => 1227775554,
    part_2 => 4174379265
}
);
