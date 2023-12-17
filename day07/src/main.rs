#[derive(Debug, Clone, Ord, PartialOrd, PartialEq, Eq)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand {
    cards: String,
    hand_type: HandType,
    bid: u32,
    use_joker: bool,
}

impl Hand {
    const CARD_LABELS: [char; 13] = [
        '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
    ];
    const CARD_LABELS_WITH_JOKER: [char; 13] = [
        'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
    ];
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            std::cmp::Ordering::Equal => {
                for card in self.cards.chars().zip(other.cards.chars()) {
                    let pos_self = match self.use_joker {
                        true => Hand::CARD_LABELS_WITH_JOKER
                            .iter()
                            .position(|&c| c == card.0)
                            .unwrap(),
                        false => Hand::CARD_LABELS.iter().position(|&c| c == card.0).unwrap(),
                    };
                    let pos_other = match other.use_joker {
                        true => Hand::CARD_LABELS_WITH_JOKER
                            .iter()
                            .position(|&c| c == card.1)
                            .unwrap(),
                        false => Hand::CARD_LABELS.iter().position(|&c| c == card.1).unwrap(),
                    };
                    if pos_self != pos_other {
                        return pos_self.cmp(&pos_other);
                    }
                }
                std::cmp::Ordering::Equal
            }
            order => order,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Hand {
    fn new(hand_data: &str, use_joker: bool) -> Self {
        let parts: Vec<&str> = hand_data.split_whitespace().collect();
        let cards = parts[0].to_string();
        let bid = parts[1].parse::<u32>().unwrap();

        // build chars count map
        let mut char_counts = std::collections::HashMap::new();
        for c in cards.chars() {
            let count = char_counts.entry(c).or_insert(0);
            *count += 1;
        }

        // Get hand type
        let hand_type = if char_counts.values().any(|&count| count == 5) {
            HandType::FiveOfAKind
        } else if char_counts.values().any(|&count| count == 4) {
            HandType::FourOfAKind
        } else if char_counts.values().any(|&count| count == 3)
            && char_counts.values().any(|&count| count == 2)
        {
            HandType::FullHouse
        } else if char_counts.values().any(|&count| count == 3) {
            HandType::ThreeOfAKind
        } else if char_counts.values().filter(|&count| *count == 2).count() == 2 {
            HandType::TwoPair
        } else if char_counts.values().any(|&count| count == 2) {
            HandType::OnePair
        } else {
            HandType::HighCard
        };

        Hand {
            cards,
            hand_type,
            bid,
            use_joker,
        }
    }
}

fn part1(input: &str) -> u64 {
    let mut hands: Vec<_> = input
        .trim()
        .split('\n')
        .map(|h| Hand::new(h, false))
        .collect::<Vec<_>>();
    hands.sort();
    hands.iter().enumerate().fold(0, |acc, (index, hand)| {
        acc + ((index + 1) as u64) * hand.bid as u64
    })
}

fn part2(input: &str) -> u64 {
    let mut hands: Vec<_> = input
        .trim()
        .split('\n')
        .map(|h| Hand::new(h, true))
        .collect();
    hands.sort();
    hands.iter().enumerate().fold(0, |acc, (index, hand)| {
        acc + ((index + 1) as u64) * hand.bid as u64
    })
}

fn main() {
    //let input = std::fs::read_to_string("input.txt").unwrap();
    let input = std::fs::read_to_string("example.txt").unwrap();
    println!("Solution of part 1 {}", part1(&input));
    println!("Solution of part 2 {}", part2(&input));
}