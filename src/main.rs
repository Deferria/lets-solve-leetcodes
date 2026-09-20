#[allow(unused_imports)]
use leetcodes::sol::four_sum;

/// Only some important debug would use the main function for better print and lint. Otherwise, run tests in the #[cfg(test)] mod tests in `sol.rs`.
fn main() {
    let v = vec![0,0,0,1000000000,1000000000,1000000000,1000000000];
    let res = four_sum(v, 1000000000);
    println!("Result: {:?}", res);
}
