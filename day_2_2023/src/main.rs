use std::str::FromStr;

fn main() {
    let input_file: &str = include_str!("../input.txt");
    let maximum_values: Vec<GameMaximums> = input_file.lines().map(get_maximum_values_part1).collect::<Vec<GameMaximums>>();
    println!("{}, {}", get_valid_ids_part1(&maximum_values).iter().sum::<u32>(), get_powers_part2(maximum_values).iter().sum::<u32>());
}

struct GameMaximums (u32, u32, u32);

fn get_maximum_values_part1(line: &str) -> GameMaximums {
    let mut maximums: [u32; 3] = [0, 0, 0];
    let game_data: Vec<&str> = line.split(": ").collect::<Vec<&str>>()[1].split("; ").collect::<Vec<&str>>();
    game_data.iter().for_each(|draws| {
        let items: Vec<&str> = draws.split(", ").collect::<Vec<&str>>();
        items.iter().for_each(|item| {
            let result = item.split(" ").collect::<Vec<&str>>();
            let count = <u32 as FromStr>::from_str(result[0]).unwrap();
            match result[1] {
                "red" => {
                    if maximums[0] < count {
                        maximums[0] = count;
                    }
                }
                "green" => {
                    if maximums[1] < count {
                        maximums[1] = count;
                    }
                }
                "blue" => {
                    if maximums[2] < count {
                        maximums[2] = count;
                    }
                }
                _ => ()
            }
        });
    });
    GameMaximums(maximums[0], maximums[1], maximums[2])
}

fn get_valid_ids_part1(games: &Vec<GameMaximums>) -> Vec<u32> {
    let mut ids: Vec<u32> = Vec::new();
    for (index, game) in games.iter().enumerate() {
        if game.0 <= 12 && game.1 <= 13 && game.2 <= 14 {
            ids.push((index + 1) as u32);
        }
    }
    ids
}

fn get_powers_part2(games: Vec<GameMaximums>) -> Vec<u32> {
    games.iter().map(|game| {
        game.0 * game.1 * game.2
    }).collect()
}