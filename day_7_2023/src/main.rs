use std::collections::HashMap;

fn main() {
    let input_file: &str = include_str!("../input.txt");

    let (mut part_1_hands, mut part_2_hands): (Vec<Hand>, Vec<Hand>) =
        input_file
            .lines()
            .map(|line| 
                line.split_once(' ').unwrap()
            ).map(|(cards, bet)| 
                (
                    parse_from_parts(cards, bet, false),
                    parse_from_parts(cards, bet, true)
                )
            )
            .unzip();
    
    part_1_hands.sort();
    part_2_hands.sort();
    println!("{}, {}", calculate_total(part_1_hands), calculate_total(part_2_hands));
}

fn get_card_value(card: char, using_jokers: bool) -> u32 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => {
            if using_jokers {
                1
            } else {
                11
            }
        },
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        _ => 2
    }
}

#[derive(Eq, PartialEq, PartialOrd, Ord)]
enum HandRank {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind
}

impl HandRank { 
    fn from_card_values(cards: &Vec<u32>, jokers: bool) -> Self {
        let mut card_count: HashMap<u32, u32> = HashMap::with_capacity(cards.len());
        let mut joker_count = 0;
        for card_value in cards {
            if jokers && *card_value == 1 {
                joker_count += 1;
            } else {
                card_count.entry(*card_value).and_modify(|n| *n += 1).or_insert(1);
            }
        }

        let instances_of_card: Vec<u32> = card_count.into_values().collect::<Vec<_>>();
        let highest_possible_card_count: u32 = instances_of_card.iter().max().unwrap_or(&0) + joker_count;
        match instances_of_card.len() {
            0 | 1 => Self::FiveOfAKind,
            2 => {
                if highest_possible_card_count == 4 {
                    Self::FourOfAKind
                } else {
                    Self::FullHouse
                }
            },
            3 => {
                if highest_possible_card_count == 3 {
                    Self::ThreeOfAKind
                } else {
                    Self::TwoPair
                }
            },
            4 => Self::OnePair,
            _ => Self::HighCard,
        }
    }
}

#[derive(PartialEq, Eq)]
struct Hand {
    cards: Vec<u32>,
    bet: u32,
    rank: HandRank
}

fn parse_from_parts(cards: &str, bid: &str, jokers: bool) -> Hand {
    let held_cards = cards.chars().map(|card_char| get_card_value(card_char, jokers)).collect::<Vec<u32>>();
    let rank = HandRank::from_card_values(&held_cards, jokers);
    Hand {
        cards: held_cards,
        bet: bid.parse::<u32>().unwrap(),
        rank
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.rank != other.rank {
            self.rank.cmp(&other.rank)
        } else {
            self.cards.cmp(&other.cards)
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn calculate_total(hands: Vec<Hand>) -> u32 {
    let mut total: u32 = 0;
    for (index, hand) in hands.iter().enumerate() {
        total += (index as u32 + 1) * hand.bet;
    }
    total
}