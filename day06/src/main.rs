fn do_races(races: &Vec<(u64, u64)>) -> u64 {
    let mut ways_to_win: Vec<u64> = Vec::new();
    for race in races {
        let mut middle = race.0 / 2;
        let mut counter = 0;
        while middle * (race.0 - middle) > race.1 {
            middle -= 1;
            counter += 1;
        }
        match race.0 % 2 {
            0 => ways_to_win.push((counter * 2) - 1),
            1 => ways_to_win.push(counter * 2),
            _ => panic!("Unexpected edge case"),
        }
    }
    ways_to_win.iter().product::<u64>()
}

fn part1() -> u64 {
    let mut lines = aoc_utils::read_strings("example_part1.txt").map(|l| l.unwrap());
    let times = lines.next().unwrap();
    let times = times
        .split_whitespace()
        .skip(1)
        .map(|t| t.parse::<u64>().unwrap());
    let distances = lines.next().unwrap();
    let distances = distances
        .split_whitespace()
        .skip(1)
        .map(|t| t.parse::<u64>().unwrap());
    let races: Vec<_> = times.zip(distances).collect();
    do_races(&races)
}

fn part2() -> u64 {
    let mut lines = aoc_utils::read_strings("example_part1.txt").map(|l| l.unwrap());
    let times = lines.next().unwrap();
    let times: String = times.chars().filter(|c| c.is_ascii_digit()).collect();
    let times = times.parse::<u64>().unwrap();
    let distances = lines.next().unwrap();
    let distances: String = distances.chars().filter(|c| c.is_ascii_digit()).collect();
    let distances = distances.parse::<u64>().unwrap();
    let races = vec![(times, distances)];
    do_races(&races)
}

fn main() {
    println!("Solution of part 1 {}", part1());
    println!("Solution of part 2 {}", part2());
}
