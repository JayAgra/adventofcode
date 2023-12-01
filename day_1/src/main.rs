use std::include_str;

fn main() {
    let input_file: &str = include_str!("../input.txt");
    let results:Vec<u32> = vec![
        input_file.lines().map(day_1_part_1).sum(),
        input_file.lines().map(day_1_part_2).sum(),
    ];
    println!("{}, {}", results.get(0).unwrap(), results.get(1).unwrap());
}

fn day_1_part_1(line: &str) -> u32 {
    // digits from line
    let mut digits = line
                                                    // get the characters
                                                    .chars()
                                                    // filter-map over it
                                                    .filter_map(|char| {
                                                        // returns none if could not convert, or the digit if successful
                                                        char.to_digit(10)
                                                    });
    // get the first digit. this will always exist
    let first_digit: u32 = digits.next().unwrap();
    
    // see if last digit exists
    let last_digit = digits.last();
    if last_digit.is_some() {
        // last digit exists, create 2 digit number
        (first_digit * 10) + last_digit.unwrap()
    } else {
        // no last digit, "double" first digit
        first_digit * 11
    }
}

// map the string elements to numbers
const NUMBER_MAPPINGS: [(&str, u32); 20] = [
    ("0", 0), ("1", 1), ("2", 2), ("3", 3), ("4", 4), ("5", 5), ("6", 6), ("7", 7), ("8", 8), ("9", 9),
    ("zero", 0), ("one", 1), ("two", 2), ("three", 3), ("four", 4), ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9)
];

fn day_1_part_2(line: &str) -> u32 {
    // create empty iterator over the line
    let mut text_numbers = (0..line.len())
                                                        // filter out all items except matches in number_mappings
                                                        .filter_map(|num| {
                                                            NUMBER_MAPPINGS
                                                                // iterate over number mappings
                                                                .iter()
                                                                // find if the substring starting at the index (num) matches an entry of the text numbers
                                                                .find(|(text_num, _value)| line[num..].starts_with(text_num))
                                                                // map these results, returns iterator
                                                                .map(|(_text_num, value)| value)
                                                        });
    // get first digit (should always exist)
    let first_digit = text_numbers.next().unwrap();
    // get last digit and unwrap because it might not exist
    let last_digit = text_numbers.last();
    if last_digit.is_some() {
        // if there is a last digit, multiply first by 10 and add last, return
        (first_digit * 10) + last_digit.unwrap()
    } else {
        // multiply first digit by 11 if no last. this is the same as putting 2 of the digit together because the number inputted is 0-9
        first_digit * 11
    }
}