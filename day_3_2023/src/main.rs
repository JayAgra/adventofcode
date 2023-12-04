use std::str::FromStr;

fn main() {
    let input_file: &str = include_str!("../input.txt");
    let schematic_grid: Vec<Vec<&str>> = input_file.lines().map(get_cleaned_row).collect();
    println!("{}, {}", get_part_numbers(schematic_grid.clone()).iter().sum::<u32>(), get_gear_ratios(schematic_grid).iter().sum::<u32>());
}

fn get_cleaned_row(line: &str) -> Vec<&str> {
    line.split("").collect::<Vec<&str>>()
}

fn safe_adjacent_element(grid: Vec<Vec<&str>>, row_index: usize, col_index: usize) -> &str {
    if row_index > grid.len() - 1 || col_index > grid[0].len() - 1 {
        "."
    } else {
        grid[row_index][col_index]
    }
}

const NUMBERS: [&str; 10] = [
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9"
];

fn is_a_symbol(element: &str) -> bool {
    element != "." && element != "" && !NUMBERS.contains(&element)
}

fn get_part_number(grid: &mut Vec<Vec<&str>>, row: usize, col: usize) -> u32 {
    let mut part_number: String = "".to_string();
    let mut position: usize = col;

    while NUMBERS.contains(&safe_adjacent_element(grid.to_vec(), row, position)) {
        position -= 1;
    };

    position += 1;

    while NUMBERS.contains(&safe_adjacent_element(grid.to_vec(), row, position)) {
        part_number = format!("{}{}", part_number, grid[row][position]);
        grid[row][position] = ".";
        position += 1;
    };

    if <u32 as FromStr>::from_str(part_number.as_str()).is_err() {
        0
    } else {
        <u32 as FromStr>::from_str(part_number.as_str()).unwrap()
    }
}

fn get_part_numbers(mut grid: Vec<Vec<&str>>) -> Vec<u32> {
    let mut part_numbers: Vec<u32> = Vec::new();
    let local_grid = grid.clone();
    for (index, line) in local_grid.iter().enumerate() {
        for (line_index, item) in line.iter().enumerate() {
            if is_a_symbol(&item) {
                let surrounding: Vec<(&str, usize, usize)> = vec![
                    (grid[index - 1][line_index - 1], index - 1, line_index - 1),
                    (grid[index - 1][line_index], index - 1, line_index),
                    (grid[index - 1][line_index + 1], index - 1, line_index + 1),
                    (grid[index][line_index - 1], index, line_index - 1),
                    (grid[index][line_index + 1], index, line_index + 1),
                    (grid[index + 1][line_index - 1], index + 1, line_index - 1),
                    (grid[index + 1][line_index], index + 1, line_index),
                    (grid[index + 1][line_index + 1], index + 1, line_index + 1),
                ];
                surrounding.iter().for_each(|adj_item: &(&str, usize, usize)| {
                    if NUMBERS.contains(&adj_item.0) {
                        part_numbers.push(get_part_number(&mut grid, adj_item.1, adj_item.2));
                    }
                });
            }
        }
    }
    part_numbers
}

fn get_gear_ratios(mut grid: Vec<Vec<&str>>) -> Vec<u32> {
    let mut gear_ratios: Vec<u32> = Vec::new();
    let local_grid = grid.clone();
    for (index, line) in local_grid.iter().enumerate() {
        for (line_index, item) in line.iter().enumerate() {
            if item == &"*" {
                let surrounding: Vec<(&str, usize, usize)> = vec![
                    (grid[index - 1][line_index - 1], index - 1, line_index - 1),
                    (grid[index - 1][line_index], index - 1, line_index),
                    (grid[index - 1][line_index + 1], index - 1, line_index + 1),
                    (grid[index][line_index - 1], index, line_index - 1),
                    (grid[index][line_index + 1], index, line_index + 1),
                    (grid[index + 1][line_index - 1], index + 1, line_index - 1),
                    (grid[index + 1][line_index], index + 1, line_index),
                    (grid[index + 1][line_index + 1], index + 1, line_index + 1),
                ];
                let mut new_ratio: Vec<u32> = Vec::new();
                surrounding.iter().for_each(|adj_item: &(&str, usize, usize)| {
                    if NUMBERS.contains(&adj_item.0) {
                        new_ratio.push(get_part_number(&mut grid, adj_item.1, adj_item.2));
                    }
                });
                new_ratio.retain(|num| num != &(0 as u32));
                if new_ratio.len() == 2 {
                    gear_ratios.push(new_ratio[0] * new_ratio[1]);
                }
            }
        }
    }
    gear_ratios
}