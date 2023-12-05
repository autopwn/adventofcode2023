fn part1() -> u32 {
    let mut sum = 0;
    for line in aoc_utils::read_strings("example_part1.txt").map(|l| l.unwrap()) {
        let (winning, holding) = &line[8..].trim().split_once('|').unwrap();
        let winning: Vec<&str> = winning.split_whitespace().collect();
        let holding: Vec<&str> = holding.split_whitespace().collect();
        let intersection: Vec<_> = winning.iter().filter(|&x| holding.contains(x)).collect();
        sum += intersection.into_iter().fold(1, |acc, _| acc * 2) / 2;
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
