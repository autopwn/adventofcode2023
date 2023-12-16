use std::fmt;

struct Pattern {
    pattern: Vec<Vec<char>>,
    horizontal: Option<usize>,
    vertical: Option<usize>,
}

impl Pattern {
    fn new(pattern: Vec<Vec<char>>) -> Self {
        Pattern {
            pattern,
            horizontal: None,
            vertical: None,
        }
    }

    fn get_score(&self) -> usize {
        self.horizontal.unwrap_or(0) * 100 + self.vertical.unwrap_or(0)
    }
}

impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _ = write!(f, "----------------------------------------\n");
        for row in 0..self.pattern.len() {
            for col in 0..self.pattern[0].len() {
                if self.horizontal.is_some()
                    && (self.horizontal.unwrap() == row || self.horizontal.unwrap() - 1 == row)
                    || self.vertical.is_some()
                        && (self.vertical.unwrap() == col || self.vertical.unwrap() - 1 == col)
                {
                    let _ = write!(f, "\x1B[32m{}\x1B[0m", self.pattern[row][col]);
                } else {
                    let _ = write!(f, "{}", self.pattern[row][col]);
                }
            }
            let _ = write!(f, "\n");
        }
        let _ = write!(
            f,
            "\nhorizontal: {:?}, vertical: {:?}, score: {}",
            self.horizontal,
            self.vertical,
            self.get_score()
        );
        Ok(())
    }
}

fn check_horizontal(pattern: &mut Pattern) {
    for row in 0..pattern.pattern.len() {
        if row > 0
            && pattern.pattern[row - 1]
                .iter()
                .zip(&pattern.pattern[row])
                .all(|(a, b)| *a == *b)
        {
            if row == 1 {
                pattern.horizontal = Some(row);
                break;
            }
            let mut offset = 1;
            loop {
                let lower_end = row.saturating_sub(offset + 1);
                let higher_end = row.saturating_add(offset);
                if higher_end >= pattern.pattern.len() {
                    pattern.horizontal = Some(row);
                    break;
                }
                if !pattern.pattern[lower_end]
                    .iter()
                    .zip(&pattern.pattern[higher_end])
                    .all(|(a, b)| *a == *b)
                {
                    break;
                }
                if lower_end == 0 {
                    pattern.horizontal = Some(row);
                    offset += 1;
                }
                offset += 1;
            }
        }
    }
}

fn get_column(pattern: &Vec<Vec<char>>, column: usize) -> Vec<&char> {
    pattern
        .iter()
        .map(|r| r.iter().nth(column).unwrap())
        .collect::<Vec<_>>()
}

fn check_vertical(pattern: &mut Pattern) {
    for col in 1..pattern.pattern[0].len() {
        if get_column(&pattern.pattern, col - 1)
            .iter()
            .zip(&get_column(&pattern.pattern, col))
            .all(|(a, b)| *a == *b)
        {
            if col == 1 {
                pattern.vertical = Some(col);
                break;
            }
            let mut offset = 1;
            loop {
                let lower_end = col.saturating_sub(offset + 1);
                let higher_end = col.saturating_add(offset);
                if higher_end >= pattern.pattern[0].len() {
                    pattern.vertical = Some(col);
                    break;
                }
                if !get_column(&pattern.pattern, lower_end)
                    .iter()
                    .zip(&get_column(&pattern.pattern, higher_end))
                    .all(|(a, b)| *a == *b)
                {
                    break;
                }
                if lower_end == 0 {
                    pattern.vertical = Some(col);
                    break;
                }
                offset += 1;
            }
        }
    }
}

fn part1() -> usize {
    //let input = std::fs::read_to_string("example_part1.txt").unwrap();
    let input = std::fs::read_to_string("input.txt").unwrap();
    let patterns: Vec<_> = input
        .split("\n\n")
        .map(|p| {
            Pattern::new(
                p.trim()
                    .split("\n")
                    .map(|l| l.trim().chars().collect::<Vec<_>>())
                    .collect::<Vec<_>>(),
            )
        })
        .collect();
    let mut sum = 0;
    for mut pattern in patterns {
        check_horizontal(&mut pattern);
        check_vertical(&mut pattern);
        sum += pattern.get_score();
        println!("{}", pattern);
    }

    sum
}

fn part2() -> u32 {
    let mut sum = 0;
    for line in aoc_utils::read_strings("example_part2.txt").map(|l| l.unwrap()) {
        println!("{}", line);
        sum += 1;
    }
    sum
}

fn main() {
    println!("Solution of part 1 {}", part1());
    println!("Solution of part 2 {}", part2());
}
