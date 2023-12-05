use rayon::prelude::*;

#[derive(Debug, Default)]
struct Almanac {
    maps: Vec<Map>,
}

#[derive(Debug, Default)]
struct Map {
    ranges: Vec<Range>,
}

#[derive(Debug, Default)]
struct Range {
    source: u64,
    destination: u64,
    length: u64,
}

fn build_almanac() -> (Vec<u64>, Almanac) {
    let mut lines = aoc_utils::read_strings("input.txt").map(|l| l.unwrap());
    let seeds = &lines.next().unwrap();
    let seeds: Vec<u64> = seeds
        .split_once(' ')
        .unwrap()
        .1
        .split(' ')
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    let mut almanac: Almanac = Default::default();
    let _ = &lines.next();
    // parse almanac
    while lines.next().is_some() {
        let mut map: Map = Default::default();
        loop {
            let line = &lines.next();
            let line = match line {
                Some(line) => line,
                None => {
                    break;
                }
            };
            if line.is_empty() {
                break;
            }
            let range: Vec<u64> = line.split(' ').map(|s| s.parse::<u64>().unwrap()).collect();
            map.ranges.push(Range {
                source: range[1],
                destination: range[0],
                length: range[2],
            });
        }
        almanac.maps.push(map);
    }
    (seeds, almanac)
}

fn part1() -> u64 {
    let (seeds, almanac) = build_almanac();

    let mut min_location = u64::MAX;
    for seed in seeds {
        let mut source = seed;
        for map in &almanac.maps {
            let mut destination = 0;
            for range in &map.ranges {
                if source >= range.source && source < range.source + range.length {
                    destination = range.destination + source - range.source;
                }
            }
            if destination != 0 {
                source = destination;
            }
        }
        if source < min_location {
            min_location = source;
        }
    }
    min_location
}

fn part2() -> u64 {
    let (seeds, almanac) = build_almanac();
    let chunks: Vec<_> = seeds.chunks(2).map(|c| c[0]..c[0]+c[1]).collect();
    // Throw CPUs at the problem :)
    let locations: Vec<u64> = chunks.par_iter().map(|chunk| {
        let mut min_location = u64::MAX;
        for seed in chunk.clone() {
            let mut source = seed;
            for map in &almanac.maps {
                let mut destination = 0;
                for range in &map.ranges {
                    if source >= range.source && source < range.source + range.length {
                        destination = range.destination + source - range.source;
                    }
                }
                if destination != 0 {
                    source = destination;
                }
            }
            if source < min_location {
                min_location = source;
            }
        }
        min_location
    }).collect();
    *locations.iter().min().unwrap()
}

fn main() {
    println!("Solution of part 1 {}", part1());
    println!("Solution of part 2 {}", part2());
}
