fn parse_game(line: &str) -> (u32, u32, u32, u32) {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    let pos_colon = line.find(':').unwrap();
    let id = line[5..pos_colon].parse().unwrap();
    for set in line[pos_colon + 2..].split(';') {
        let set = set.trim();
        for cube in set.split(',') {
            let (count, cube) = cube.trim().split_once(' ').unwrap();
            let count = count.parse().unwrap();
            match cube.chars().next() {
                Some('r') if count > red => red = count,
                Some('g') if count > green => green = count,
                Some('b') if count > blue => blue = count,
                Some(_) => {}
                _ => panic!(),
            }
        }
    }
    (id, red, green, blue)
}

fn part1() {
    let mut sum = 0;
    for line in aoc_utils::read_strings("example_part1.txt").map(|l| l.unwrap()) {
        let game = parse_game(&line);
        if game.1 <= 12 && game.2 <= 13 && game.3 <= 14 {
            sum += game.0;
        }
    }
    println!("Sum of game ids {}", sum);
}

fn part2() {
    let mut sum = 0;
    for line in aoc_utils::read_strings("example_part2.txt").map(|l| l.unwrap()) {
        let game = parse_game(&line);
        sum += game.1 * game.2 * game.3;
    }
    println!("Sum of game ids {}", sum);
}

fn main() {
    part1();
    part2();
}
