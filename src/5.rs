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

fn getline_as_u32_vec() -> Vec<u32> {
    let l = getline();
    let nlv: Vec<_> = l.trim().split(' ').collect();
    nlv.into_iter().map(|x| x.parse::<u32>().unwrap()).collect()
}

fn main() {
    let l = getline_as_i32() as u32;
    let n = getline_as_i32() as u32;

    // 参照として誰頭につかわせたいときに所有権を考える
    let mut mwv = getline_as_u32_vec();

    // Vecとまったく同じものを再生成してしまっている
    // let mut mwv = Vec::from(wv);

    mwv.sort();

    let mut count: u32 = 0;
    let mut sum: u32 = 0;

    // ここで所有権を失う
    // mwv.into_iter().for_each(|v| {
    //     sum += v;
    //     if sum >= l {
    //         println!("{}", count);
    //     }
    //     count += 1;
    // });

    for i in 0..n {
        sum += mwv[i as usize];
        if sum > l {
            break;
        }
        count += 1;
    }
    println!("{}", count);
}
