use procoio::input;

fn main() {
    input! {
        n: i32,
        w: i32,
    }

    println!("{}", n / w);
}
