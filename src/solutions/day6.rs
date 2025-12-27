crate::day!("Trash Compactor" => {
    part_1,
    part_2
});

fn part_1(data: &str) -> isize {
    let lines: Vec<&str> = data.lines().filter(|l| !l.trim().is_empty()).collect();
    let (number_rows, op_row) = lines.split_at(lines.len() - 1);

    let ops: Vec<char> = op_row[0]
        .split_whitespace()
        .map(|tok| tok.chars().next().unwrap())
        .collect();

    let mut total = 0;

    for (col, &op) in ops.iter().enumerate() {
        let mut iter = number_rows.iter().map(|row| {
            row.split_whitespace()
                .nth(col)
                .unwrap()
                .parse::<isize>()
                .unwrap()
        });

        let mut value = iter.next().unwrap();
        match op {
            '+' => {
                for x in iter {
                    value += x;
                }
            }
            '*' => {
                for x in iter {
                    value *= x;
                }
            }
            _ => unreachable!(),
        }

        total += value;
    }

    total
}

fn part_2(data: &str) -> isize {
    0
}

crate::test_day!(

"
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
",
{
    part_1 => 4277556,
    part_2 => 0
}
);
