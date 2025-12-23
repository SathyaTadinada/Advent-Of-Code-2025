crate::day!("Printing Department" => {
    part_1,
    part_2
});

fn part_1(data: &str) -> isize {
    let mut count = 0;
    let lines: Vec<&str> = data.trim().lines().collect();
    let height = lines.len();
    let width = lines[0].len();

    let padded_height = height + 2;
    let padded_width = width + 2;

    let mut grid = vec![b'.'; padded_height * padded_width];

    for (r, line) in lines.iter().enumerate() {
        for (c, ch) in line.bytes().enumerate() {
            grid[(r + 1) * padded_width + (c + 1)] = ch;
        }
    }

    let dirs = [
        (-1, -1),   (-1, 0),    (-1, 1),
        (0, -1),                (0, 1),
        (1, -1),    (1, 0),     (1, 1),
    ];

    for y in 1..=height {
        for x in 1..=width {
            let index = y * padded_width + x;
            if grid[index] != b'@' {
                continue;
            }

            let mut adj = 0;
            for (dy, dx) in dirs {
                let n_index =
                    ((y as isize) + dy) as usize * padded_width + ((x as isize) + dx) as usize;
                if grid[n_index] == b'@' {
                    adj += 1;
                }
            }
            if adj < 4 {
                count += 1;
            }
        }
    }

    count
}

fn part_2(data: &str) -> isize {
    let mut count = 0;
    let lines: Vec<&str> = data.trim().lines().collect();
    let height = lines.len();
    let width = lines[0].len();

    let padded_height = height + 2;
    let padded_width = width + 2;

    let mut grid = vec![b'.'; padded_height * padded_width];

    for (r, line) in lines.iter().enumerate() {
        for (c, ch) in line.bytes().enumerate() {
            grid[(r + 1) * padded_width + (c + 1)] = ch;
        }
    }

    let dirs = [
        (-1, -1),   (-1, 0),    (-1, 1),
        (0, -1),                (0, 1),
        (1, -1),    (1, 0),     (1, 1),
    ];
    loop {
        let mut to_remove: Vec<usize> = Vec::new();
        for y in 1..=height {
            for x in 1..=width {
                let index = y * padded_width + x;
                if grid[index] != b'@' {
                    continue;
                }

                let mut adj = 0;
                for (dy, dx) in dirs {
                    let n_index =
                        ((y as isize) + dy) as usize * padded_width + ((x as isize) + dx) as usize;
                    if grid[n_index] == b'@' {
                        adj += 1;
                    }
                }
                if adj < 4 {
                    to_remove.push(index);
                }
            }
        }
        if to_remove.is_empty() {
            break;
        }
        count += to_remove.len() as isize;
        for idx in to_remove {
            grid[idx] = b'.';
        }
    }

    count
}

crate::test_day!(

"
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
",
{
    part_1 => 13,
    part_2 => 43
}
);
