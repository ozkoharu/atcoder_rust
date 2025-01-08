use std::io;

fn main() {
    //標準入力から１行読み込み
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");

    //  3つの整数をベクタに格納
    // 空白のスペースを削除
    // 各部分文字列を クロージャ |x| x.parse.expext(...)によって整数に変換
    // イテレータからデータを収集し、新しいコレクション型にしま

    let mut group: Vec<i32> = input
    .split_whitespace()
    .map(|x| x.parse().expect("Failed to parse integer"))
    .collect();

    // 昇順ソート
    group.sort();

    if group[0] == group[1] && group[1] == group[2] {
        println!("Yes");
    } else if group[2] == group[0] + group[1]  {
        println!("Yes");
    } else {
        println!("No");
    }

}
