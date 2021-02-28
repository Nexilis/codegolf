// compile with: rustc 99bottles.rs
// run with ./99bottles
pub fn main() {
    let b = "of beer on the wall";
    println!("99 bottles {}, 99 bottles of beer.", b);

    (0..99).rev().for_each(|x| {
        let r = f(x);
        let t = r.to_lowercase();
        println!("Take one down and pass it around, {} {}.\n\n{} {}, {} of beer.",t, b, r, b, t);
    });
    println!("Go to the store and buy some more, 99 bottles {}.", b);
}

fn f(x: u8) -> String {
    match x {
        0 => "No more bottles".to_string(),
        1 => "1 bottle".to_string(),
        _ => format!("{} bottles", x)
    }
}
