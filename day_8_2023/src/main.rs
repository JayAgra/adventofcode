fn main() {
    let mut input_file: std::str::Lines<'_> = include_str!("../input.txt").lines();
    let directions: Vec<char> = input_file.next().unwrap().chars().collect();
    input_file.next();
    let mut input_shit: Vec<NodeLabel> = input_file.map(|line| NodeLabel::from_str(line)).collect();
    input_shit.sort_by_key(|k: &NodeLabel| k.id);
    println!("{}, {}", from_to(&directions, &input_shit, 0, &is_max_id), part_2(directions, input_shit));
}

#[derive(Debug)]
struct NodeLabel {
    id: u32,
    direction: (u32, u32)
}

impl NodeLabel {
    fn from_str(line: &str) -> Self {
        let parts = line.split_once(" = (").unwrap();
        let directions = parts.1.split_once(", ").unwrap();
        NodeLabel {
            id: letter_id_to_numid(parts.0),
            direction: (letter_id_to_numid(directions.0), letter_id_to_numid(directions.1.trim_end_matches(")"))) 
        }
    }
}

fn letter_id_to_numid(letter: &str) -> u32 {
    let mut chars = letter.chars();
    let mut numerical: u32 = 0;
    numerical += (chars.next().unwrap().to_ascii_uppercase() as u32 - 65) * 676;
    numerical += (chars.next().unwrap().to_ascii_uppercase() as u32 - 65) * 26;
    numerical += (chars.next().unwrap().to_ascii_uppercase() as u32 - 65) * 1;
    numerical
}

fn is_max_id(current: u32) -> bool {
    current == 17575
}

fn from_to(directions: &Vec<char>, input_shit: &Vec<NodeLabel>, start_id: u32, end_condition: &dyn Fn(u32) -> bool) -> usize {
    let directions_length = directions.len();
    let mut current_id: u32 = start_id;
    let mut current_step: usize = 0;
    while !end_condition(current_id) {
        current_step += 1;
        let direction_index = (current_step - 1) % directions_length;
        let result_ind = input_shit.binary_search_by(|f| f.id.cmp(&current_id)).unwrap();
        let result = &input_shit[result_ind];
        if directions[direction_index] == 'L' {
            current_id = result.direction.0;
        } else {
            current_id = result.direction.1;
        }
    }
    current_step
}

fn is_end_in_z(current: u32) -> bool {
    (current - 25) % 26 == 0
}

fn greatest_common_denominator(a: usize, b: usize) -> usize {
    let (mut a, mut b) = (a, b);
    
    if b > a {
        let t = a;
        a = b;
        b = t;
    }

    loop {
        let res = a % b;
        if res == 0 {
            return b;
        }
        a = b;
        b = res;
    }
}					

fn part_2(directions: Vec<char>, input_shit: Vec<NodeLabel>) -> usize {
    let current_ids: Vec<usize> = input_shit.iter().filter(|x| x.id % 26 == 0).map(|f| f.id).map(|f| from_to(&directions, &input_shit, f, &is_end_in_z)).collect();
    let mut result: usize = 1;
    current_ids.iter().for_each(|f| {
        result = (result * f) / greatest_common_denominator(result, *f);
    });
    result
}