fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let ll = getline();
    let ml = getline();
    let nl = getline();
    let alv: Vec<_> = ll.trim().split(' ').collect();
    let mlv: Vec<_> = ml.trim().split(' ').collect();
    let nlv: Vec<_> = nl.trim().split(' ').collect();
    let l: u32 = alv[0].parse().unwrap();
    let m: u32 = mlv[0].parse().unwrap();
    let n: u32 = nlv[0].parse().unwrap();

    let num_25: u32 = n / 25 + m;
    let mod_1 = n % 25;
    let num_100 = num_25 * 25 / 100 + l;
    let mod_25 = (num_25 * 25 % 100) / 25;
    let mod_100 = num_100 * 100 % 1000 / 100;

    println!("{}", mod_1 + mod_25 + mod_100);

    // let input = &args[0];
    // let range = 1..=input.parse::<u64>().unwrap();
    //
    // println!("{}", range.sum::<u4>());
}
