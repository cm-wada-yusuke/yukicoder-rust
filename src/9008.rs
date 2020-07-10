fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let s1 = getline();
    let s2 = getline();
    let e1: Vec<_> = s1.trim().split(' ').collect();
    let e2: Vec<_> = s2.trim().split(' ').collect();
    let n: i32 = e1[0].parse().unwrap();
    let mut sum: u64 = 0;

    for i in 0..n {
        let num = e2[i as usize].parse::<u64>().unwrap();
        sum += num
    }

    println!("{}", sum)
}
