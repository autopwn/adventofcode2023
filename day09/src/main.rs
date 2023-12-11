fn iterate(numbers: &Vec<i64>, front: bool) -> i64 {
    if numbers.iter().all(|n| *n == 0) {
        return 0;
    }
    let next_numbers: Vec<_> = numbers.windows(2).map(|a| a[1].saturating_sub(a[0])).collect();
    let result = iterate(&next_numbers, front);
    match front {
        true => numbers.first().unwrap() - result,
        false => numbers.last().unwrap() + result
    }
}

fn part1() -> i64 {
    aoc_utils::read_strings("input.txt").map(|l| l.unwrap()).map(|line| {
        iterate(line.split_whitespace().map(|c| c.parse::<i64>().unwrap()).collect::<Vec<i64>>().as_ref(), false)
    }).sum()
}

fn part2() -> i64 {
    aoc_utils::read_strings("input.txt").map(|l| l.unwrap()).map(|line| {
        iterate(line.split_whitespace().map(|c| c.parse::<i64>().unwrap()).collect::<Vec<i64>>().as_ref(), true)
    }).sum()
}

fn main() {
    println!("Solution of part 1 {}", part1());
    println!("Solution of part 2 {}", part2());
}
