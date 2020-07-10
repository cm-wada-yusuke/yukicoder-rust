fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let s1 = getline();
    let e1: Vec<_> = s1.trim().split(' ').collect();
    let n: i32 = e1[0].parse().unwrap();
    let mut sum: u64 = 0;

    for _ in 0..n {
        let s = getline();
        let e: Vec<_> = s.trim().split(' ').collect();
        let num = e[0].parse::<u64>().unwrap();
        sum += num
    }

    println!("{}", sum)
}
