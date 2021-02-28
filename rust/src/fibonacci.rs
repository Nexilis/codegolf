// compile with: rustc fibonacci.rs
// run with ./fibonacci
pub fn main() {
    let mut i = 0;
    let mut j = 1;
    for _ in 0..31 {
        println!("{}", i);
        let k = i + j;
        i = j;
        j = k;
    };
}
