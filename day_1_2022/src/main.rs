use std::{include_str, str::FromStr};

fn main() {
    let input_file: &str = include_str!("../input.txt");
    let elf_totals = get_elves(input_file);
    println!("{}, {}", get_top_elf_value(&elf_totals), get_top_three_elves(&elf_totals));
}

fn get_elves(input_file: &str) -> Vec<u32> {
    let elves: Vec<&str> = input_file.split("\n\n").collect();
    let mut elf_totals: Vec<u32> = Vec::new();
    elves.iter().for_each(|elf_string| {
        let elf_value_str: Vec<&str> = elf_string.split("\n").collect();
        let elf_values: Vec<u32> = elf_value_str.into_iter().map(|val| <u32 as FromStr>::from_str(val).unwrap()).collect();
        elf_totals.push(elf_values.iter().sum());
    });
    elf_totals
}

fn get_top_elf_value(elf_totals: &Vec<u32>) -> u32 {
    elf_totals
        .iter()
        .max()
        .unwrap()
        .clone()
}

fn get_top_three_elves(elf_totals_input: &Vec<u32>) -> u32 {
    let mut elf_totals: Vec<u32> = elf_totals_input.clone();
    elf_totals.sort();
    let total_elves: usize = elf_totals.len();
    return elf_totals[total_elves - 1] + elf_totals[total_elves - 2] + elf_totals[total_elves - 3];
}