fn getline() -> String{
    let mut __ret=String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn main() {
    let s=getline();
    let a:Vec<_>=s.trim().split(' ').collect();
    let x:i32=a[0].parse().unwrap();
    print!("{} ",(1..=x).sum::<i32>());

    // let input = &args[0];
    // let range = 1..=input.parse::<u64>().unwrap();
    //
    // println!("{}", range.sum::<u64>());
}
