// The first naiv attempt with modelling the trench in a 2d grid
// and getting the area by flood filling or bfs was not suitable for part2 (typical aoc surprise)
// Solution for part2 was to use the shoelace formula and picks theorem but forever reason with a tweak

fn part(lines: &[&str], part2: bool) -> i64 {
    const DIRECTIONS: [char; 4] = ['R', 'D', 'L', 'U'];
    let mut points: Vec<(i64, i64)> = Vec::with_capacity(lines.len() + 1);
    let mut cur: (i64, i64) = (0, 0);
    points.push(cur);
    let mut trench_length = 0;
    for line in lines {
        let mut instructions = line.split_whitespace();
        let direction = instructions.next().unwrap().chars().next().unwrap();
        let length: i64 = instructions.next().unwrap().parse().unwrap();
        let (length, direction) = match part2 {
            // part 1
            false => (
                length,
                DIRECTIONS.iter().position(|c| *c == direction).unwrap(),
            ),
            // part 2
            true => {
                let mut color = instructions
                    .next()
                    .unwrap()
                    .chars()
                    .filter(|c| !['(', ')', '#'].contains(c));
                let length =
                    i64::from_str_radix(&color.by_ref().take(5).collect::<String>(), 16).unwrap();
                (length, color.next().unwrap().to_digit(10).unwrap() as usize)
            }
        };
        trench_length += length;
        cur = match direction {
            3 => (cur.0 - length, cur.1),
            1 => (cur.0 + length, cur.1),
            2 => (cur.0, cur.1 - length),
            0 => (cur.0, cur.1 + length),
            _ => panic!(),
        };
        points.push(cur);
    }

    // Area of polygon using shoelace formula
    let inner_area = points
        .windows(2)
        .fold(0, |acc, edge| {
            acc + edge[0].0.checked_mul(edge[1].1).unwrap()
                - edge[0].1.checked_mul(edge[1].0).unwrap()
        })
        .abs()
        / 2;

    // Picks theorem, but with +1 instead of -1 ???
    inner_area + (trench_length / 2) + 1
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let lines = input.trim().split('\n').collect::<Vec<_>>();

    println!("Solution of part 1 {}", part(&lines, false));
    println!("Solution of part 2 {}", part(&lines, true));
}
