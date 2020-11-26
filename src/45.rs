use std::cmp::max;

fn getline() -> String {
    let mut __ret = String::new();
    std::io::stdin().read_line(&mut __ret).ok();
    return __ret;
}

fn getline_as_u32() -> u32 {
    let l = getline();
    let nlv: Vec<_> = l.trim().split(' ').collect();
    nlv[0].parse::<u32>().unwrap()
}

fn getline_as_u32_vec() -> Vec<u32> {
    let l = getline();
    let nlv = l.trim().split(' ');
    nlv.map(|x| x.parse::<u32>().unwrap()).collect()
}

fn main() {
    // 一つ前の状態を使う活動計画問題として考える
    let n: usize = getline_as_u32() as usize;
    let vv: Vec<u32> = getline_as_u32_vec();

    // DPテーブル > dp[i+1] := i番目の皿までの品物の中からおいしさの最大値
    let mut dp: Vec<u32> = (0..=n).map(|_| u32::MIN).collect();
    dp[0] = 0;
    dp[1] = vv[0];

    // DPループ iがDPの状態
    for i in 1..n {
        let taberu: u32 = dp[i - 1] + vv[i];
        let taberarenai: u32 = dp[i];
        dp[i + 1] = max(taberu, taberarenai);
    }

    println!("{}", dp[n]);
}
