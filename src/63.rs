fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

// 一行のスペース区切りを u64 のベクタで取得する 例：1 2 3 4 4 => <1, 2, 3, 4>
fn getline_as_u64_vec() -> Vec<u64> {
    let l = getline();
    let nlv: Vec<_> = l.trim().split(' ').collect();
    nlv.into_iter().map(|x| x.parse::<u64>().unwrap()).collect()
}

fn main() {
    // let mut lk = vec![3, 4, 5, 6, 1, 2]; // あらかじめミュータブルとして宣言しておく
    //
    // lk.sort(); // sort は所有権を要求しないので、 lk はこのあとも使えるが、中身をいじられている

    let lk = getline_as_u64_vec();
    let l = lk[0];
    let k = lk[1];

    // 唇が触れるないようにするというのをマイナス1することで表現する
    // あとは唇が触れないポッキーを何回食べられるかという割り算
    let times: u64 = (l - 1) / (k * 2);

    println!("{}", k * times);
}
