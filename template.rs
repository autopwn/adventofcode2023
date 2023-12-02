fn part1() -> u32 {
    let mut sum = 0;
    for line in aoc_utils::read_strings("example_part1.txt").map(|l| l.unwrap()) {
        println!("{}", line);
        sum += 1;
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
