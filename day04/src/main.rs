fn split_card(card: &str) -> (Vec<&str>, Vec<&str>) {
    let (winning, holding) = &card[8..].trim().split_once('|').unwrap();
    let winning: Vec<_> = winning.trim().split_whitespace().collect();
    let holding: Vec<_> = holding.trim().split_whitespace().collect();
    (winning, holding)
}

fn count_matches(card: &str) -> u64 {
    let (winning, holding) = split_card(card);
    winning.iter().filter(|&x| holding.contains(&x)).count() as u64
}

fn part1(cards: &Vec<&str>) -> u64 {
    let mut sum = 0;
    for card in cards {
        let (winning, holding) = split_card(card);
        let intersection: Vec<_> = winning.iter().filter(|&x| holding.contains(x)).collect();
        sum += intersection.into_iter().fold(1, |acc, _| acc * 2) / 2;
    }
    sum
}

fn part2(cards: &Vec<&str>) -> u64 {
    let mut overall_count: usize = 0;
    for number in 0..cards.len() {
        let mut stack: Vec<usize> = Vec::with_capacity(30);
        let mut count: usize = 0;
        stack.push(number);
        while !stack.is_empty() {
            let card_number = stack.pop().unwrap();
            count += 1;
            let match_count = count_matches(cards[card_number]);
            for card_number in card_number + 1..=(card_number + match_count as usize) {
                stack.push(card_number);
            }
        }
        overall_count += count;
    }
    overall_count as u64
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let cards: Vec<_> = input.trim().split("\n").collect();
    println!("Solution of part 1 {}", part1(&cards));
    println!("Solution of part 2 {}", part2(&cards));
}
