fn main() {
    let days = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut happy_day: u32 = 0;

    for month in 1..=12 {
        for day in 1..=days[month as usize - 1] {
            let day_char = format!("{:02}", day).to_string().chars().collect::<Vec<char>>();
            if month == day_char[0].to_string().parse::<i32>().unwrap() + day_char[1].to_string().parse::<i32>().unwrap() {
                happy_day += 1
            }
        }
    }

    println!("{}", happy_day)
}
