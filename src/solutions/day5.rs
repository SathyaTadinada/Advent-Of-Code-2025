use std::collections::HashSet;

crate::day!("Cafeteria" => {
    part_1,
    part_2
});

fn part_1(data: &str) -> isize {
    let mut count = 0;
    let (ranges_part, nums_part) = data.trim().split_once("\n\n").unwrap();
    let ranges = ranges_part
        .lines()
        .map(|line| {
            let (a, b) = line.split_once('-').unwrap();
            (a.parse::<isize>().unwrap(), b.parse::<isize>().unwrap())
        })
        .collect::<Vec<_>>();
    let nums = nums_part
        .lines()
        .map(|line| line.parse::<isize>().unwrap())
        .collect::<Vec<_>>();

    for num in nums {
        for (start, end) in &ranges {
            if num < *start || num > *end {
                continue;
            }
            if num >= *start && num <= *end {
                count += 1;
                break;
            }
        }
    }
    count
}

fn part_2(data: &str) -> isize {
    let mut count = 0;
    let (ranges_part, _nums_part) = data.trim().split_once("\n\n").unwrap();
    let mut ranges = ranges_part
        .lines()
        .map(|line| {
            let (a, b) = line.split_once('-').unwrap();
            (a.parse::<isize>().unwrap(), b.parse::<isize>().unwrap())
        })
        .collect::<Vec<_>>();

    ranges.sort_by_key(|&(start, _end)| start);
    
    let mut current_start = ranges[0].0;
    let mut current_end = ranges[0].1;

    for &(start, end) in ranges.iter().skip(1) {
        if start <= current_end + 1 {
            current_end = current_end.max(end);
        } else {
            count += current_end - current_start + 1;
            current_start = start;
            current_end = end;
        }
    }

    count + current_end - current_start + 1
}

crate::test_day!(

"
3-5
10-14
16-20
12-18

1
5
8
11
17
32
",
{
    part_1 => 3,
    part_2 => 14
}
);
