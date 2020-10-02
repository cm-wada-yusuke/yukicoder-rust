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
    let a: String = getline_as_str();
    let b: String = getline_as_str();

    // ソートした結果同じかどうか判定すればよい
    let mut achars = a.chars().collect::<Vec<char>>();
    achars.sort();
    let astr = achars.into_iter().collect::<String>();

    let mut bchars = b.chars().collect::<Vec<char>>();
    bchars.sort();
    let bstr = bchars.into_iter().collect::<String>();

    println!("{}", if astr == bstr { "YES" } else { "NO" })
}
