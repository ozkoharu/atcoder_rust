use proconio::input;

fn main() {
    // 1行ごとにまとめて受け取る
    input! {
        n: usize,
        c1: char,
        c2: char,
        s: String,
    }

    // 文字列sのうち、c１以外をc２に置き換える
    // s.chars()で(chars型)イテレータを取得
    // |ch|にはmapで回す値が入ります。
    let result:String = s.chars().map(|ch| if ch == c1 { ch } else { c2 }).collect();

    println!("{}", result);
}
