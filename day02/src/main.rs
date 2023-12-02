fn parse_game(line: &str) -> (u32, u32, u32, u32) {
    let (mut r, mut g, mut b) = (0, 0, 0);
    let pos_colon = line.find(':').unwrap();
    let id = line[line.find(' ').unwrap() + 1..pos_colon]
        .parse()
        .unwrap();
    for set in line[pos_colon + 2..].split(';') {
        let set = set.trim();
        for cube in set.split(',') {
            let (count, cube) = cube.trim().split_once(' ').unwrap();
            let count = count.parse().unwrap();
            match cube.chars().next() {
                Some('r') => {
                    if count > r {
                        r = count
                    }
                }
                Some('g') => {
                    if count > g {
                        g = count
                    }
                }
                Some('b') => {
                    if count > b {
                        b = count
                    }
                }
                _ => panic!("Unexcpected cube name"),
            }
        }
    }
    (id, r, g, b)
}

fn part1() -> u32 {
    aoc_utils::read_strings("example_part1.txt")
        .map(|l| parse_game(&l.unwrap()))
        .filter(|g| g.1 <= 12 && g.2 <= 13 && g.3 <= 14)
        .map(|g| g.0)
        .sum()
}

fn part2() -> u32 {
    aoc_utils::read_strings("example_part2.txt")
        .map(|l| parse_game(&l.unwrap()))
        .map(|g| g.1 * g.2 * g.3)
        .sum()
}

fn main() {
    println!("Solution of part 1 {}", part1());
    println!("Solution of part 2 {}", part2());
}
