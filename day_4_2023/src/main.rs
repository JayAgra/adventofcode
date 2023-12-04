use std::str;

fn main() {
    let input_file: &str = include_str!("../input.txt");
    let card_wins = get_win_count(input_file.lines().collect::<Vec<&str>>());
    println!(
        "{}, {}", 
        calc_total_points_part_1(&card_wins),
        calc_clone_cards_part_2(&card_wins)
    );
}

fn get_win_count(cards: Vec<&str>) -> Vec<u32> {
    let mut card_wins: Vec<u32> = Vec::new();
    cards.iter().for_each(|card| {
        let values: Vec<&str> = card.split(":").collect::<Vec<&str>>()[1].split(" |").collect::<Vec<&str>>();
        let (winning_numbers, your_numbers): (Vec<&str>, Vec<&str>) = 
            (
                values[0].as_bytes().chunks(3).map(str::from_utf8).collect::<Result<Vec<&str>, _>>().unwrap(),
                values[1].as_bytes().chunks(3).map(str::from_utf8).collect::<Result<Vec<&str>, _>>().unwrap()
            );
        let mut card_matches: u32 = 0;
        your_numbers.iter().for_each(|value| {
            if winning_numbers.contains(value) {
                card_matches += 1;
            }
        });
        card_wins.push(card_matches);
    });
    card_wins
}

fn calc_total_points_part_1(cards: &Vec<u32>) -> u32 {
    cards.iter().map(|card| {
        if card == &(0 as u32) {
            0
        } else {
            u32::pow(2, card - 1)
        }
    }).sum::<u32>()
}

fn calc_clone_cards_part_2(cards: &Vec<u32>) -> u32 {
    let mut card_count: Vec<u32> = vec![1; cards.len()];
    for (index, card) in cards.iter().enumerate() {
        for i in index + 1..=(index + *card as usize) {
            card_count[i] += card_count[index];
        }
    };
    card_count.iter().sum()
}