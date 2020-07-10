fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let s = getline();
    let a: Vec<_> = s.trim().split(' ').collect();
    let A: u64 = a[0].parse().unwrap();
    let B: u64 = a[1].parse().unwrap();

    for n in A..=B {
        let day_char = n.to_string().chars().collect::<Vec<char>>();
        if day_char.contains(&'3') || n % 3 == 0 {
            println!("{}", n);
        }
    }

    // let input = &args[0];
    // let range = 1..=input.parse::<u64>().unwrap();
    //
    // println!("{}", range.sum::<u4>());
}
