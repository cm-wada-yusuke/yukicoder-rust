fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn getline_as_u32_vec() -> Vec<u32> {
    let l = getline();
    let nlv = l.trim().split(' ');
    nlv.map(|x| x.parse::<u32>().unwrap()).collect()
}

fn main() {
    let bs: Vec<u32> = getline_as_u32_vec();

    // 書かなかった数がひとつだけだと確定しているので本来の合計値55から引いてあげればOK
    println!("{}", 55 - bs.into_iter().sum::<u32>());
}
