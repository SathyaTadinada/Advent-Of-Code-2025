crate::day!("Historian Hysteria" => {
    part_1,
    part_2
});

fn part_1(data: &str) -> isize {
    println!("Hi");
    0
}

fn part_2(data: &str) -> isize {
    println!("Hello");
    0
}

crate::test_day!(
"
3   4
4   3
2   5
1   3
3   9
3   3
",
{
    part_1 => 11,
    part_2 => 13
}
);
