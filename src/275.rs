fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn getline_as_u64() -> u64 {
    let l = getline();
    let nlv: Vec<_> = l.trim().split(' ').collect();
    nlv[0].parse::<u64>().unwrap()
}

// 一行のスペース区切りを u64 のベクタで取得する 例：1 2 3 4 4 => <1, 2, 3, 4>
fn getline_as_i64_vec() -> Vec<i64> {
    let l = getline();
    let nlv: Vec<_> = l.trim().split(' ').collect();
    nlv.into_iter().map(|x| x.parse::<i64>().unwrap()).collect()
}

fn main() {
    let n = getline_as_u64();
    let mut av = Vec::from(getline_as_i64_vec());
    av.sort();

    if n % 2 == 0 {
        let m1 = ((n - 1) as f64 / 2.0).floor() as usize;
        let m2 = ((n - 1) as f64 / 2.0).ceil() as usize;
        println!("{:.1}", (av[m1] + av[m2]) as f64 / 2.0)
    } else {
        println!("{:.1}", av[((n - 1) / 2) as usize] as f64)
    }

    // println!("{:.1}", origins.join(""))

    // let input = &args[0];
    // let range = 1..=input.parse::<u64>().unwrap();
    //
    // println!("{}", range.sum::<u4>());
}
