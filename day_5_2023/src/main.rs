use std::str::Lines;

#[derive(Debug, Clone)]
struct Attribute {
    pub destination: u32,
    pub source: u32,
    pub length: u32
}

fn main() {
    let mut input_file: Lines<> = include_str!("../input.txt").lines();
    let seeds: Vec<u32> = input_file.next().unwrap().split_once(":").unwrap().1.split(" ").map(|seed| str::parse::<u32>(seed).unwrap_or(0)).collect::<Vec<u32>>();
    let attribute_maps = input_file
                        .clone()
                        .filter(|line| !line.is_empty())
                        .fold(vec![] as Vec::<Vec::<Attribute>>, |mut atr_map_accumulator: Vec::<Vec::<Attribute>>, line| -> Vec::<Vec::<Attribute>> {
                            if line.contains( "map:") {
                                atr_map_accumulator.push(vec![]);
                            } else {
                                let last_index = atr_map_accumulator.len() - 1;
                                let mut value_iterator = line.split(" ").map(|value| str::parse::<u32>(value).unwrap());
                                atr_map_accumulator[last_index].push(
                                    Attribute {
                                        destination: value_iterator.next().unwrap(),
                                        source: value_iterator.next().unwrap(),
                                        length: value_iterator.next().unwrap()
                                    }
                                );
                            }
                            atr_map_accumulator
                        });
    println!("{}", process_seeds(&seeds, attribute_maps.clone()));
}

fn process_seeds(seeds: &Vec<u32>, attribute_maps: Vec<Vec<Attribute>>) -> u32 {
    seeds
        .clone()
        .into_iter()
        .map(|val| attribute_maps.iter().fold(val, |value, attr_items| {
            match attr_items.iter().find(|attribute| value >= attribute.source && value < attribute.source + attribute.length) {
                Some(attribute) => attribute.destination - attribute.source + value,
                None => value
            }
        }))
        .collect::<Vec<_>>()
        .iter()
        .min()
        .unwrap()
        .clone()
}