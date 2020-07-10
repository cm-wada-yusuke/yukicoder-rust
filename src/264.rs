fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let s = getline();
    let a: Vec<_> = s.trim().split(' ').collect();
    let n: i32 = a[0].parse().unwrap();
    let k: i32 = a[1].parse().unwrap();

    match (n, k) {
        (0, 1) | (1, 2) | (2, 0) =>
            println!("Won"),
        (1, 0) | (2, 1) | (0, 2) =>
            println!("Lost"),
        _ => println!("Drew")
    }

    // let input = &args[0];
    // let range = 1..=input.parse::<u64>().unwrap();
    //
    // println!("{}", range.sum::<u4>());
}
