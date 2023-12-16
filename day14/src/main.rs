use std::fmt;

#[derive(Clone)]
struct Platform {
    data: Vec<Vec<char>>,
}

impl Platform {
    fn new(data: Vec<Vec<char>>) -> Self {
        Platform { data }
    }
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..self.data.len() {
            for col in 0..self.data[0].len() {
                let _ = write!(f, "{}", self.data[row][col]);
            }
            let _ = write!(f, "\n");
        }
        Ok(())
    }
}

fn part1(platform: &mut Platform) -> usize {
    // Boundary per column which rolling rocks cannot pass
    let mut north_boundary: Vec<usize> = vec![0; platform.data.len()];
    // Tilt platform
    for row in 0..platform.data.len() {
        for col in 0..platform.data[0].len() {
            match platform.data[row][col] {
                'O' => {
                    // roll rock to position before current boundary and increment boundary
                    if north_boundary[col] != row {
                        platform.data[north_boundary[col]][col] = 'O';
                        platform.data[row][col] = '.';
                    }
                    north_boundary[col] += 1;
                },
                '#' => {
                    // move boundary to cupe-shaped rock
                    north_boundary[col] = row + 1;
                },
                _ => {}
            }
        }
    }
    println!("{}", platform);
    // Calculate load
    let platform_len = platform.data.len();
    let mut sum = 0;
    for row in 0..platform_len {
        for col in 0..platform.data[0].len() {
            if platform.data[row][col] == 'O' {
                sum += platform_len - row;
            }
        }
    }

    sum
}

fn part2(platform: &mut Platform) -> u32 {
    0
}

fn main() {
    //let input = std::fs::read_to_string("input.txt").unwrap();
    let input = std::fs::read_to_string("example_part1.txt").unwrap();
    let data = input
        .trim()
        .split("\n")
        .map(|l| l.trim().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut platform = Platform::new(data);

    println!("Solution of part 1 {}", part1(&mut platform.clone()));
    println!("Solution of part 2 {}", part2(&mut platform));
}
