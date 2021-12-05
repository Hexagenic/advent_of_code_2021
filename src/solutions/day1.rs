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

    let a_it = lines.iter();
    let b_it = lines.iter().skip(1);
    let c_it = lines.iter().skip(2);
    let rolling_window = a_it.zip(b_it).zip(c_it);

    for ((a, b), c) in rolling_window {
        let line = a + b + c;
        if let Some(last_line) = last_line {
            if line > last_line {
                count += 1;
            }
        }
        last_line = Some(line);
    }

    count
}
