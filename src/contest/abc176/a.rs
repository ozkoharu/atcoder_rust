use proconio::input;

fn main() {
    input! {
        n: i32,
        x: i32,
        t: i32,
    }
    println!("{}", t * ((n - 1) / x + 1));
}
