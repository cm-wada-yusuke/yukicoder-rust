fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn getline_as_u64_vec() -> Vec<u64> {
    let l = getline();
    let nlv = l.trim().split(' ');
    nlv.map(|x| x.parse::<u64>().unwrap()).collect()
}

fn main() {
    let is = getline_as_u64_vec();
    // let n = is[0];
    // let h = is[1];
    // let m = is[2];
    // let t = is[3];

    // let vec![n, h, m, t] = is;
    let [n, h, m, t] = [is[0], is[1], is[2], is[3]];

    // すべて分になおして加算する
    let minutes = h * 60 + m + (n - 1) * t;

    println!("{}", minutes / 60 % 24);
    println!("{}", minutes % 60);
}
