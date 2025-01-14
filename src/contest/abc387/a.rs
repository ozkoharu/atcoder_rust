use std::io;

fn main() {
    // 標準入力から１行読み込み
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");

    let mut a: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().expect("failed to parse interger"))
        .collect();

    let sum = a[0] + a[1];
    let ans = sum * sum;
    println!("{}", ans);
}
