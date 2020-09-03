fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn getline_as_str() -> String {
    let l = getline();
    let nlv: Vec<_> = l.trim().split(' ').collect();
    String::from(nlv[0])
}

fn main() {
    let s: String = getline_as_str();
    let cs = s.chars();

    // イテレータのメソッドを有効活用していくスタイル
    println!(
        "{}",
        cs.map(|c| { c.to_digit(10).unwrap_or(0) }).sum::<u32>()
    );
}
