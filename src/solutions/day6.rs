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

    let rows: Vec<Vec<isize>> = number_rows
        .iter()
        .map(|row| {
            row.split_whitespace()
                .map(|tok| tok.parse::<isize>().unwrap())
                .collect()
        })
        .collect();

    let mut total: isize = 0;

    for (col, &op) in ops.iter().enumerate() {
        let mut value = rows[0][col];
        match op {
            '+' => {
                for row in &rows[1..] {
                    value += row[col];
                }
            }
            '*' => {
                for row in &rows[1..] {
                    value *= row[col];
                }
            }
            _ => unreachable!(),
        }
        total += value;
    }

    total
}

fn part_2(data: &str) -> isize {
    let lines: Vec<&str> = data.lines().filter(|l| !l.trim().is_empty()).collect();
    let height = lines.len();
    let width = lines.iter().map(|l| l.len()).max().unwrap_or(0);

    let cell = |row: &str, col: usize| -> u8 {
        row.as_bytes().get(col).copied().unwrap_or(b' ')
    };

    let mut is_separator_col = vec![false; width];
    for (col, separator) in is_separator_col.iter_mut().enumerate() {
        *separator = lines.iter().all(|row| cell(row, col) == b' ');
    }

    let mut blocks: Vec<(usize, usize)> = Vec::new();
    let mut col = 0;
    while col < width {
        while col < width && is_separator_col[col] {
            col += 1;
        }
        if col >= width {
            break;
        }
        let start = col;
        while col < width && !is_separator_col[col] {
            col += 1;
        }
        let end = col;
        blocks.push((start, end));
    }

    let mut total = 0;

    for (start, end) in blocks {
        let op_row = lines[height - 1];
        let mut op = None;
        for c in start..end {
            let ch = cell(op_row, c);
            if ch == b'+' || ch == b'*' {
                op = Some(ch as char);
                break;
            }
        }
        let op = op.expect("operator not found in block");

        let mut numbers: Vec<i128> = Vec::new();

        for c in (start..end).rev() {
            let mut digits: Vec<u8> = Vec::new();

            for row in &lines[0..height - 1] {
                let ch = cell(row, c);
                if ch.is_ascii_digit() {
                    digits.push(ch - b'0');
                }
            }

            if !digits.is_empty() {
                let mut value: i128 = 0;
                for d in digits {
                    value = value * 10 + d as i128;
                }
                numbers.push(value);
            }
        }

        let block_value: i128 = match op {
            '+' => numbers.iter().sum(),
            '*' => numbers.iter().product(),
            _ => unreachable!(),
        };

        total += block_value;
    }

    total as isize
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
    part_2 => 3263827
}
);
