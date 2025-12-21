crate::day!("Secret Entrance" => {
    part_1,
    part_2
});

fn part_1(data: &str) -> isize {
    let mut dial = 50;
    let mut count = 0;

    for line in data.trim().lines() {
        let direction: char = line[0..1].parse().unwrap();
        // println!("Direction: {}", direction);
        let distance: isize = line[1..].parse().unwrap();
        // println!("Distance: {}", distance);

        if direction == 'L' {
            dial -= distance;
        } else {
            dial += distance;
        }

        dial = (dial + 100) % 100;
        if dial == 0 {
            count += 1;
        }

    }
    count as isize
}

fn part_2(data: &str) -> isize {
    let mut dial: isize = 50;
    let mut count: isize = 0;

    for line in data.trim().lines() {
        let direction: char = line[0..1].parse().unwrap();
        let distance: isize = line[1..].parse().unwrap();

        if direction == 'L' {
            if dial != 0 {
                if distance >= dial {
                    count += 1 + (distance - dial) / 100;
                }
            } else {
                count += distance / 100;
            }

            dial -= distance;
            while dial < 0 {
                dial += 100;
            }
        } else {
            let steps_to_zero = (100 - dial) % 100;
            if steps_to_zero != 0 {
                if distance >= steps_to_zero {
                    count += 1 + (distance - steps_to_zero) / 100;
                }
            } else {
                count += distance / 100;
            }

            dial += distance;
            while dial >= 100 {
                dial -= 100;
            }
        }
    }

    count
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
    part_2 => 6
}
);
