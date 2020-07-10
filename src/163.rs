fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

// 与えられた文字列を char のベクタに変換します
fn to_char_verctor(str: &str) -> Vec<char> {
    str.chars().collect::<Vec<char>>()
}

fn main() {
    let ll = getline();
    let alv: Vec<_> = ll.trim().split(' ').collect();
    let l: &str = alv[0];

    let passwords = to_char_verctor(l);

    let origins: Vec<String> = passwords
        .iter()
        .map(|c| {
            if c.is_uppercase() {
                c.to_ascii_lowercase().to_string()
            } else {
                c.to_ascii_uppercase().to_string()
            }
        })
        .collect();

    println!("{}", origins.join(""))

    // let input = &args[0];
    // let range = 1..=input.parse::<u64>().unwrap();
    //
    // println!("{}", range.sum::<u4>());
}
