use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
        c: u32,
        x: u32,
    }

    let mut ans = 0;
    for i in 0..(a + 1) {
        for j in 0..(b + 1) {
            for k in 0..(c + 1) {
                if 500 * i + 100 * j + 50 * k == x {
                    ans += 1;
                }
            }
        }
    }

    println!("{}", ans);
}
