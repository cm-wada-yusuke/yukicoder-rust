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
    let wv: Vec<u32> = getline_as_u32_vec();

    // dp[i番目までのおもり][片方の重さ] := i番目のおもりを使って片方の重さjが存在するか
    let mut dp: Vec<Vec<bool>> = (0..=n)
        .map(|_| (0..=n * 100).map(|_| false).collect())
        .collect();
    dp[0][0] = true;

    // DPループ iがDPの状態
    for i in 0..n {
        for j in 0..n * 100 {
            // DP 表 dp[i番目までのおもり][片方の重さ] を true にできるか

            // ひとつまえの選択状態をキャッチして、乗せるパターンとのせないパターンを記録する
            if dp[i][j] == true {
                // 選んだ場合、一つ前の重さ（J）ｊに i のおもりを加える
                dp[i + 1][j + wv[i] as usize] = true;
                // のせない場合、その重さが継続される
                dp[i + 1][j] = true;
            }
        }
    }

    // 全部の重さ/2になるようなDPが存在すればOK
    let sum = wv.into_iter().sum::<u32>();
    if sum % 2 == 0 && dp[n][sum as usize / 2] {
        println!("possible");
    } else {
        println!("impossible");
    }
}
