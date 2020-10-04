use proconio::input;

fn main() {
    input! {
        n: u32,
        y: u32,
    }

    for i in (0..(n + 1)).rev() {
        for j in (0..(n - i + 1)).rev() {
            let k = n - i - j;
            if 10000 * i + 5000 * j + 1000 * k == y {
                println!("{} {} {}", i, j, k);
                return;
            }
        }
    }

    println!("-1 -1 -1");
}
