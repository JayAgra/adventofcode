fn main() {
    let input_file: &str = include_str!("../input.txt");
    let histories: Vec<SandHistory> = get_histories(input_file);
    println!("{}, {}", get_predictions(&histories), get_part_2_predictions(&histories));
}

struct SandHistory {
    tree: Vec<Vec<i32>>,
}

fn get_histories(lines: &str) -> Vec<SandHistory> {
    let mut histories: Vec<SandHistory> = Vec::new();
    lines.lines().for_each(|line| {
        let actual_history: Vec<i32> = line.split(" ").map(|f|  f.parse::<i32>().unwrap()).collect();
        let mut tree_builder: Vec<Vec<i32>> = Vec::new();
        tree_builder.push(actual_history);
        let mut new_vec: Vec<i32> = Vec::new();
        for i in 1..tree_builder[0].len() {
            new_vec.push(tree_builder[0][i] - tree_builder[0][i - 1]);
        }
        tree_builder.push(new_vec);
        while !tree_builder.last().unwrap().iter().all(|v| v == &0) {
            let last_vec: &Vec<i32> = tree_builder.last().unwrap();
            let mut new_vec: Vec<i32> = Vec::new();
            for i in 1..last_vec.len() {
                new_vec.push(last_vec[i] - last_vec[i - 1]);
            }
            tree_builder.push(new_vec);
        }
        histories.push(SandHistory {
            tree: tree_builder
        });
    });
    histories
}

fn get_predictions(histories: &Vec<SandHistory>) -> i32 {
    let mut predictions: i32 = 0;
    histories.iter().for_each(|sand| {
        let mut inc_amount: i32 = 0;
        sand.tree.iter().rev().for_each(|history| {
            inc_amount += history.last().unwrap();
        });
        predictions += inc_amount;
    });
    predictions
}

fn get_part_2_predictions(histories: &Vec<SandHistory>) -> i32 {
    let mut predictions: i32 = 0;
    histories.iter().for_each(|sand| {
        let mut inc_amount: i32 = 0;
        sand.tree.iter().rev().for_each(|history| {
            inc_amount = history.first().unwrap() - inc_amount;
        });
        predictions += inc_amount;
    });
    predictions
}