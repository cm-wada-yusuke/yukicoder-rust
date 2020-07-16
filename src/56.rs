fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn getline_as_u64_vec() -> Vec<u64> {
    let l = getline();
    let nlv: Vec<_> = l.trim().split(' ').collect();
    nlv.into_iter().map(|x| x.parse::<u64>().unwrap()).collect()
}

fn main() {
    let dp = getline_as_u64_vec();
    let d = dp[0] as f64;
    let p = dp[1] as f64;

    println!("{}", (d + (d * p / 100.0).floor()) as u64)
}
