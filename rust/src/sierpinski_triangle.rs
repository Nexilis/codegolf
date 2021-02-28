// clear && rustfmt sierpinski_triangle.rs && rustc sierpinski_triangle && ./sierpinski_triangle

pub fn main() {
    let base: i32 = 2;
    let height: i32 = base.pow(4); // 16
    let width: i32 = base.pow(5); // 32

    let canvas = vec![vec![0; 31]; 16];
    for mut y in canvas {
        y[3] = 1;
        println!("{:?}", y);
    }

    for y in 0..height {
        for x in 0..width - 1 {
            if is_bottom(x, y, height) {
                print!("b")
            } else if is_left(x, y, height) {
                print!("l");
            } else if is_right(x, y, height) {
                print!("r");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn is_bottom(x: i32, y: i32, height: i32) -> bool {
    y == height - 1 && x % 2 == 0
}

fn is_left(x: i32, y: i32, height: i32) -> bool {
    x == height - y - 1
}

fn is_right(x: i32, y: i32, height: i32) -> bool {
    x == height + y - 1
}
