fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn getline_as_u32_vec() -> Vec<u32> {
    let l = getline();
    let nlv: Vec<_> = l.trim().split(' ').collect();
    nlv.into_iter().map(|x| x.parse::<u32>().unwrap()).collect()
}

fn getline_as_u64_vec() -> Vec<u64> {
    let l = getline();
    let nlv: Vec<_> = l.trim().split(' ').collect();
    nlv.into_iter().map(|x| x.parse::<u64>().unwrap()).collect()
}

fn main() {
    let knf = getline_as_u64_vec();
    let av = getline_as_u64_vec();
    let k = knf[0];
    let n = knf[1];
    let f = knf[2];

    let mame_sum: u64 = k * n;
    let eat_sum: u64 = av.iter().fold(0, |sum, a| sum + *a);

    if (mame_sum < eat_sum) {
        println!("-1")
    } else {
        println!("{}", mame_sum - eat_sum);
    }
}
