use std::io;

fn main() {
    // 標準入力から１行読み込み
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");

    let trimmed_input = input.trim();
    let char_ary: Vec<char> = trimmed_input.chars().collect();
    let upc = "UPC";
    let ans = format!("{}{}", char_ary[0], upc);

    println!("{}", ans);
}
