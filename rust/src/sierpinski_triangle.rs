// clear && rustfmt sierpinski_triangle.rs && rustc sierpinski_triangle.rs && ./sierpinski_triangle

pub fn main() {
    let base: i32 = 2;
    let height: i32 = base.pow(4); // 16
    let width: i32 = base.pow(5); // 32

    let mut canvas = vec![vec![' '; (width - 1) as usize]; height as usize];

    for y in 0..height {
        for x in 0..width - 1 {
            let xu = x as usize;
            let yu = y as usize;
            if is_bottom(x, y, height) {
                triangle_at(&mut canvas, xu, yu);
            } else if is_left(x, y, height) {
                triangle_at(&mut canvas, xu, yu);
            } else if is_right(x, y, height) {
                triangle_at(&mut canvas, xu, yu);
            }
        }
    }
    for y in canvas {
        println!("{:?}", y);
    }
}

fn triangle_at(canvas: &mut Vec<Vec<char>>, x: usize, y: usize) {
    let row = &mut canvas[y];
    row[x] = '▲';
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
