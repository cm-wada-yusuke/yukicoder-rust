fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn getline_as_i64() -> i64 {
    let l = getline();
    let nlv: Vec<_> = l.trim().split(' ').collect();
    nlv[0].parse::<i64>().unwrap()
}

fn main() {
    let x: i64 = getline_as_i64();
    let y: i64 = getline_as_i64();
    let l: i64 = getline_as_i64();

    // 向きの変更に何手かかるかを先に考える。もっとシンプルにできますか？
    let mut order_count = if y < 0 {
        2
    } else if x != 0 {
        1
    } else {
        0
    };

    // あとは絶対値の距離だけ切り上げ除算
    order_count += (x.abs() + (l - 1)) / l + (y.abs() + (l - 1)) / l;

    println!("{}", order_count)
}
