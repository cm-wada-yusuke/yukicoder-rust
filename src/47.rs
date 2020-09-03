fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn getline_as_u64() -> u64 {
    let l = getline();
    let nlv: Vec<_> = l.trim().split(' ').collect();
    nlv[0].parse::<u64>().unwrap()
}

fn main() {
    let n: u64 = getline_as_u64();
    let mut remain = Some(n);
    let mut pocket: u64 = 1;
    let mut hit = 0;

    loop {
        // まずはポケットを叩いてみるッス
        let next = pocket * 2;
        match remain {
            Some(1) => remain = None,
            Some(some_remain) => {
                if some_remain == next {
                    // ポケットの中身と残数が同じになったらポンとやって終了
                    hit = hit + 1;
                    remain = None;
                } else if some_remain > next {
                    // それでも残数のほうが多い場合、もう一度やる
                    hit = hit + 1;
                    pocket = next
                } else {
                    // ポケットの中身のほうが多くなってしまう場合、そのままたたかずにポケットの中身をひとつ減らす
                    remain = Some(some_remain - 1);
                    pocket = pocket - 1;
                }
            }
            _ => {
                break;
            }
        }
    }

    println!("{}", hit)
}
