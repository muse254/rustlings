// clippy2.rs
// Make me compile! Execute `rustlings hint clippy2` for hints :)

fn main() {
    let mut res = 42;
    let option = Some(12);
    // solution: **Example:** `for x in option { .. }`. This should be `if let Some(x) = option { .. }`.
    if let Some(x) = option {
        res += x;
    }
    println!("{}", res);
}
