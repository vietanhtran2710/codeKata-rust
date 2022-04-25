use std::fs;

fn process_data(filename: &str, total_col: usize, column1: usize, column2: usize, result_column: usize, max: bool) -> (String, f32) {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.split("\n").collect();
    let (mut result, mut value) = ("", if max {0.0} else {100.0});
    for item in lines.iter().skip(2) {
        let data: Vec<&str> = item
            .split(" ")
            .filter(|item| item != &"")
            .collect();
        if data.len() < total_col { continue; }
        let difference = (data[column1].parse::<f32>().unwrap() - data[column2].parse::<f32>().unwrap()).abs();
        (result, value) = if (max && difference > value) || (!max && difference < value) {(data[result_column], difference)} else {(result, value)};
    }
    (String::from(result), value)
}

fn main() {
    let (day_number, max) = process_data("weather.dat", 10, 1, 2, 0, true);
    println!("Day {} has the widest spread of {}", day_number, max);

    let (team, min) = process_data("football.dat", 10, 6, 8, 1, false);
    println!("Team {} has smallest difference {}", team, min);
}
