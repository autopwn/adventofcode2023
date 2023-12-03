use aoc_utils::read_char_matrix;
use std::collections::HashMap;

fn extract_part_number(
    engine: &[Vec<char>],
    visited_part_numbers: &mut HashMap<(usize, usize), bool>,
    r: usize,
    e: usize,
) -> Option<u32> {
    // part number already visited
    if visited_part_numbers.contains_key(&(r, e)) {
        return None;
    }

    // extract left part of part number
    let left: String = engine[r][0..e]
        .iter()
        .rev()
        .enumerate()
        .take_while(|(_, c)| c.is_ascii_digit())
        .map(|(u, c)| {
            let pos = e - u - 1;
            visited_part_numbers.entry((r, pos)).or_insert(true);
            c
        })
        .collect();
    let left: String = left.chars().rev().collect();

    // extract rigth part of part number
    let right: String = engine[r][e..]
        .iter()
        .enumerate()
        .take_while(|(_, c)| c.is_ascii_digit())
        .map(|(u, c)| {
            let pos = e + u;
            visited_part_numbers.entry((r, pos)).or_insert(true);
            c
        })
        .collect();

    // stick part number together and parse string into number
    Some(format!("{}{}", left, right).parse::<u32>().unwrap())
}

fn part1() -> u32 {
    let engine = read_char_matrix("example_part1.txt");
    let mut visited_part_numbers: HashMap<(usize, usize), bool> = HashMap::new();
    let mut sum = 0;
    for (r, row) in engine.iter().enumerate() {
        for (e, &element) in row.iter().enumerate() {
            if !element.is_ascii_digit() && element != '.' {
                for (i, _) in engine
                    .iter()
                    .enumerate()
                    .take((r + 1).min(engine.len() - 1) + 1)
                    .skip(r.saturating_sub(1))
                {
                    for j in (e.saturating_sub(1))..=(e + 1).min(engine[i].len() - 1) {
                        // Skip the target element itself
                        if (i != r || j != e) && engine[i][j].is_ascii_digit() {
                            if let Some(part_number) =
                                extract_part_number(&engine, &mut visited_part_numbers, i, j)
                            {
                                sum += part_number
                            }
                        }
                    }
                }
            }
        }
    }
    sum
}

fn part2() -> u32 {
    let engine = read_char_matrix("example_part2.txt");
    let mut visited_part_numbers: HashMap<(usize, usize), bool> = HashMap::new();
    let mut sum = 0;
    for (r, row) in engine.iter().enumerate() {
        for (e, &element) in row.iter().enumerate() {
            if element == '*' {
                let mut part_numbers = Vec::new();
                for (i, _) in engine
                    .iter()
                    .enumerate()
                    .take((r + 1).min(engine.len() - 1) + 1)
                    .skip(r.saturating_sub(1))
                {
                    for j in (e.saturating_sub(1))..=(e + 1).min(engine[i].len() - 1) {
                        // Skip the target element itself
                        if (i != r || j != e) && engine[i][j].is_ascii_digit() {
                            if let Some(part_number) =
                                extract_part_number(&engine, &mut visited_part_numbers, i, j)
                            {
                                part_numbers.push(part_number);
                            }
                        }
                    }
                }
                if part_numbers.len() == 2 {
                    sum += part_numbers.into_iter().fold(1, |acc, x| acc * x);
                }
            }
        }
    }
    sum
}

fn main() {
    println!("Solution of part 1 {}", part1());
    println!("Solution of part 2 {}", part2());
}
