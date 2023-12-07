fn main() {
    let input_file: &str = include_str!("../input.txt");
    let records: RaceRecords = get_records(input_file);
    println!("{}, {}", calc_part1(&records), calc_part2(&records));
}

struct RaceRecords {
    time: Vec<i64>,
    distance: Vec<i64>,
}

fn get_records(input: &str) -> RaceRecords {
    let mut input_lines = input.lines();
    RaceRecords {
        time: input_lines.next().unwrap().split_once("Time:        ").unwrap().1.split("     ").map(|time| str::parse::<i64>(time).unwrap()).collect::<Vec<i64>>(),
        distance: input_lines.next().unwrap().split_once("Distance:   ").unwrap().1.split("   ").map(|dist: &str| str::parse::<i64>(dist).unwrap()).collect::<Vec<i64>>()
    }
}

fn hold_time(distance: i64, time: i64) -> i64 {
    (-1 * ( ( -1.0 * ( time as f64 ) + ( ( time.pow(2) as f64 ) - ( 4.0 * ( distance as f64 ) ) ).sqrt() ) as i64 / 2 ) ) as i64
}

fn calc_possible_wins(time: i64, distance: i64) -> i64 {
    let optimal_rel: f64 = time as f64 / 2.0;
    (optimal_rel as i64 - (hold_time(distance, time))) * 2 - ((optimal_rel % 1.0 + 0.5).powi(-1) - 1.0) as i64
}

fn calc_part1(records: &RaceRecords) -> i64 {
    let mut total = 1;
    for i in 0..4 {
        total *= calc_possible_wins(records.time[i], records.distance[i]);
    }
    total
}

fn calc_part2(records: &RaceRecords) -> i64 {
    calc_possible_wins(
        records.time.iter().map(|num| num.to_string()).collect::<Vec<String>>().concat().parse::<i64>().unwrap(),
        records.distance.iter().map(|num| num.to_string()).collect::<Vec<String>>().concat().parse::<i64>().unwrap()
    )
}