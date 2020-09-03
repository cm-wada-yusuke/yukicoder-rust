use chrono::Local;
use std::io::Split;

fn main() {
    println!("Hello, world! {}", Local::today());
}

fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn getline_as_i32() -> i32 {
    let l = getline();
    let nlv: Vec<_> = l.trim().split(' ').collect();
    nlv[0].parse::<i32>().unwrap()
}

fn getline_as_u64() -> u64 {
    let l = getline();
    let nlv: Vec<_> = l.trim().split(' ').collect();
    nlv[0].parse::<u64>().unwrap()
}

// 一行の単語ひとつ文字列そのまま取得する 例：abcdefg
fn getline_as_str() -> String {
    let l = getline();
    let nlv: Vec<_> = l.trim().split(' ').collect();
    String::from(nlv[0])
}

// 一行のスペース区切りを u64 のベクタで取得する 例：1 2 3 4 4 => <1, 2, 3, 4>
fn getline_as_u64_vec() -> Vec<u64> {
    let l = getline();
    let nlv = l.trim().split(' ');
    nlv.map(|x| x.parse::<u64>().unwrap()).collect()
}

fn getline_as_u32_vec() -> Vec<u32> {
    let l = getline();
    let nlv = l.trim().split(' ');
    nlv.map(|x| x.parse::<u32>().unwrap()).collect()
}

// 与えられた文字列を char のベクタに変換します
fn to_char_vector(str: &str) -> Vec<char> {
    str.chars().collect::<Vec<char>>()
}

// 一行の単語ひとつ
fn getline_as_char_vector() -> Vec<char> {
    let line = getline_as_str();
    to_char_vector(&line)
}
