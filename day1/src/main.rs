use utils::read_lines;

fn main() {
    let mut result = get_calories();
    result.sort();

    let outcome1 = result.iter().rev().next().unwrap();
    let outcome2: usize = result.iter().rev().take(3).sum();

    println!("The most Calories: {}", outcome1);
    println!("The total is: {}", outcome2);
}

fn get_calories() -> Vec<usize> {
    let mut temp_value = 0;
    let mut result: Vec<usize> = Vec::new();
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(l) = line {
                if let Ok(value) = l.parse::<usize>() {
                    temp_value += value;
                } else {
                    result.push(temp_value);
                    temp_value = 0;
                }
            }
        }
        result.push(temp_value);
    }
    result
}
