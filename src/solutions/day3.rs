crate::day!("Lobby" => {
    part_1,
    part_2
});

fn part_1(data: &str) -> isize {
    let mut sum = 0;

    for line in data.trim().lines() {
        let digits: Vec<isize> = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as isize)
            .collect();
        if digits.len() < 2 {
            continue;
        }

        let mut max_after = *digits.last().unwrap();
        let mut best = 0;

        for i in (0..digits.len() - 1).rev() {
            best = best.max(digits[i] * 10 + max_after);
            max_after = max_after.max(digits[i]);
        }

        sum += best;
    }

    sum
}

fn part_2(data: &str) -> isize {
    let total: i128 = data
        .trim()
        .lines()
        .map(|line| best_k_digits(line.trim(), 12))
        .sum();
    total as isize
}

fn best_k_digits(line: &str, k: usize) -> i128 {
    let digits: Vec<u8> = line.bytes().map(|b| b - b'0').collect();
    let mut drop = digits.len().saturating_sub(k);
    let mut stack: Vec<u8> = Vec::with_capacity(digits.len());

    for d in digits {
        while drop > 0 && !stack.is_empty() && *stack.last().unwrap() < d {
            stack.pop();
            drop -= 1;
        }
        stack.push(d);
    }

    while drop > 0 {
        stack.pop();
        drop -= 1;
    }

    let mut val: i128 = 0;
    for &d in stack.iter().take(k) {
        val = val * 10 + d as i128;
    }
    val
}

crate::test_day!(

"
987654321111111
811111111111119
234234234234278
818181911112111
",
{
    part_1 => 357,
    part_2 => 3121910778619
}
);
