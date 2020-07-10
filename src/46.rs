fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let s1 = getline();
    let e1: Vec<_> = s1.trim().split(' ').collect();
    let a: u64 = e1[0].parse::<u64>().unwrap();
    let b: u64 = e1[1].parse::<u64>().unwrap();

    let walk =  (b as f64 / a as f64).ceil();

    println!("{}", walk)


}
