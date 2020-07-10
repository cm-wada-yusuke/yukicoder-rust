fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let s1 = getline();
    let e1: Vec<_> = s1.trim().split(' ').collect();
    let s: u64 = e1[0].parse::<u64>().unwrap();
    let f: u64 = e1[1].parse::<u64>().unwrap();


    if s % f == 0 {
        println!("{}", s / f + 1);
    } else if s < f {
        println!("1");
    } else {
        println!("{}", (s as f64 / f as f64).ceil());
    }
}
