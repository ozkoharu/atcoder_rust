use std::io;

fn main() {
    // 標準入力から1行読み込み
    let mut input = String::new();

    // &mut はinput の参照
    // read_lineはResult型を返すのでexpectでエラーメッセージを出す
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // 4つの整数をベクタに格納
    let mut a: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().expect("Filed to parse interger."))
        .collect();

    // 昇順ソート
    a.sort();

    // 条件分岐
    if a[0] == a[1] && a[1] ==a[2] && a[2] != a[3] {
        println!("Yes");
    } else if a[0] != a[1] && a[1] == a[2] && a[2] == a[3] {
        println!("Yes");
    } else if a[0] == a[1] && a[1] != a[2] && a[2] == a[3] {
        println!("Yes");
    } else  {
        println!("No");
    }

 }