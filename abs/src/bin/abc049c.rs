use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let patterns = ["eraser", "erase", "dreamer", "dream"];
    let r = patterns.iter().fold(s, |t, x| t.replace(x, ""));
    println!("{}", if r.is_empty() { "YES" } else { "NO " })
}
