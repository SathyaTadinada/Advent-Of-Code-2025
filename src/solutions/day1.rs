crate::day!("Secret Entrance" => {
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
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
",
{
    part_1 => 3,
    part_2 => 13
}
);
