use std::collections::HashMap;
use num::integer::lcm;

fn parse() -> (String, HashMap<String, (String, String)>) {
    let mut map: HashMap<String, (String, String)> = HashMap::new();
    let mut lines_iter =  aoc_utils::read_strings("input.txt").map(|l| l.unwrap());
    // get instructions
    let instructions = lines_iter.next().unwrap();
    let _ = lines_iter.next();
    // fill map
    for line in lines_iter {
        map.insert(line[0..3].to_string(), (line[7..10].to_string(), line[12..15].to_string()));
    }
    (instructions, map)
}

fn part1() -> u64 {
    let (instructions, map) = parse();
    let mut count = 0;
    let mut current = "AAA".to_string();
    for instruction in instructions.chars().cycle() {
        current = match instruction {
            'R' => map.get(&current).unwrap().1.clone(),
            'L' => map.get(&current).unwrap().0.clone(),
            _ => panic!("unexpected instruction")
        };
        count += 1;
        if current == "ZZZ" {
            break;
        }
    }
    count
}

fn part2() -> u64 {
    let (instructions, map) = parse();
    let mut count: u64 = 0;
    let mut current: Vec<&str> = map.keys().filter(|p| p.ends_with("A")).map(|p| p.as_ref()).collect();
    let mut numbers: Vec<u64> = Vec::default();
    for instruction in instructions.chars().cycle() {
        current = current.iter().map(|c| {
            match instruction {
                'R' => map.get(*c).unwrap().1.as_ref(),
                'L' => map.get(*c).unwrap().0.as_ref(),
                _ => panic!("unexpected instruction")
            }
        }).collect();
        count += 1;
        current = current.into_iter().filter(|f| {
            if f.ends_with("Z") {
                numbers.push(count);
                return false;
            }
            true
        }).collect();

        if current.is_empty() {
            break;
        }
    }
    numbers.iter().fold(numbers[0], |acc, &x| lcm(acc, x))
}

fn main() {
    println!("Solution of part 1 {}", part1());
    println!("Solution of part 2 {}", part2());
}
