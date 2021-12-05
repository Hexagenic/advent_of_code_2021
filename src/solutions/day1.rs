pub fn part_a(file: &str) -> i64 {
    let lines = file.split_whitespace().flat_map(str::parse);
    let mut last_line: Option<i32> = None;

    let mut count = 0;

    for line in lines {
        if let Some(last_line) = last_line {
            if line > last_line {
                count += 1;
            }
        }
        last_line = Some(line);
    }

    count
}

pub fn part_b(file: &str) -> i64 {
    let lines: Vec<i32> = file.split_whitespace().flat_map(str::parse).collect();

    let mut last_line: Option<i32> = None;

    let mut count = 0;

    for vals in lines.windows(3) {
        let line = vals.iter().sum();
        if let Some(last_line) = last_line {
            if line > last_line {
                count += 1;
            }
        }
        last_line = Some(line);
    }

    count
}
