use std::io;

fn main() {
    // 標準入力から文字列を変換
    let  mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    // 1文字目を数字として受け取る
    // .chars()は文字列を文字(Chars型)に変換するメソッド
    // イテレータは次の要素がないかもしれないからOptions型を返すことから
    //  unwrap()する必要がある。
    let first = input.chars().nth(0).unwrap().to_digit(10).unwrap();
    let third = input.chars().nth(2).unwrap().to_digit(10).unwrap();

    let ans = first * third;
    println!("{}", ans);
}
