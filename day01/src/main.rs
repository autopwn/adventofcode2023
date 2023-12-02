fn find_digits(line: &str, digit_words: &[&str]) -> (u32, u32) {
    let mut first: Option<u32> = Option::default();
    let mut last: Option<u32> = Option::default();
    for pos in 0..line.len() {
        match line.chars().nth(pos) {
            Some(c) if c.is_ascii_digit() => {
                last = Some(c as u32 - '0' as u32);
                if first.is_none() {
                    first = Some(c as u32 - '0' as u32);
                }
            }
            _ => {
                let remainder = &line[pos..];
                for (index, word) in digit_words.iter().enumerate() {
                    if remainder.starts_with(word) {
                        last = Some((index + 1) as u32);
                        if first.is_none() {
                            first = Some((index + 1) as u32);
                        }
                    }
                }
            }
        }
    }
    if first.is_none() || last.is_none() {
        panic!("no digit found");
    }
    (first.unwrap(), last.unwrap())
}

fn part1() -> u32 {
    aoc_utils::read_strings("example_part1.txt")
        .map(|line| {
            let line = line.unwrap();
            let mut digits = line
                .chars()
                .filter(|c| c.is_ascii_digit());
            let first = digits.next().unwrap();
            // edge case when there is only one digit in the line
            let last = digits.last().unwrap_or(first);
            format!("{}{}", first, last)
                .parse::<u32>()
                .unwrap()
        })
        .sum()
}

fn part2() -> u32 {
    let digit_words: Vec<&str> = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    aoc_utils::read_strings("example_part2.txt")
        .map(|line| {
            let (first, last) = find_digits(&line.unwrap(), &digit_words);
            format!("{}{}", first, last).parse::<u32>().unwrap()
        })
        .sum()
}

fn main() {
    println!("Solution of part 1 {}", part1());
    println!("Solution of part 2 {}", part2());
}
