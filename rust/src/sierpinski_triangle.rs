// clear && rustfmt sierpinski_triangle.rs && rustc sierpinski_triangle.rs && ./sierpinski_triangle

// my best code.golf solution for now:
// pub fn main(){for i in vec![15,-1,14,1,-1,13,3,-1,12,1,1,1,-1,11,7,-1,10,1,5,1,-1,9,3,3,3,-1,8,1,1,1,1,1,1,1,-1,7,15,-1,6,1,13,1,-1,5,3,11,3,-1,4,1,1,1,9,1,1,1,-1,3,7,7,7,-1,2,1,5,1,5,1,5,1,-1,1,3,3,3,3,3,3,3,-1,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1]{if i==-1{println!()}else{for _ in 0..i{print!(" ")}print!("▲")}}}

pub fn main() {
    let base: i32 = 2;
    let width: i32 = base.pow(5); // 32
    let height: i32 = base.pow(4); // 16

    let mut canvas = vec![vec![' '; (width - 1) as usize]; height as usize];
    draw_single_triangle_at(&mut canvas, 0, width, 0, height);

    for y in canvas {
        println!("{:?}", y);
    }

    let compressed = vec![
        15, -1, 14, 1, -1, 13, 3, -1, 12, 1, 1, 1, -1, 11, 7, -1, 10, 1, 5, 1, -1, 9, 3, 3, 3, -1,
        8, 1, 1, 1, 1, 1, 1, 1, -1, 7, 15, -1, 6, 1, 13, 1, -1, 5, 3, 11, 3, -1, 4, 1, 1, 1, 9, 1,
        1, 1, -1, 3, 7, 7, 7, -1, 2, 1, 5, 1, 5, 1, 5, 1, -1, 1, 3, 3, 3, 3, 3, 3, 3, -1, 0, 1, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ];

    for i in compressed {
        if i == -1 {
            println!();
        } else {
            for _ in 0..i {
                print!(" ");
            }
            print!("▲");
        }
    }
}

fn draw_single_triangle_at(
    canvas: &mut Vec<Vec<char>>,
    start_width: i32,
    width: i32,
    start_height: i32,
    height: i32,
) {
    for y in start_height..height {
        for x in start_width..width - 1 {
            let xu = x as usize;
            let yu = y as usize;

            if is_bottom(x, y, height) {
                triangle_at(canvas, xu, yu);
            } else if is_left(x, y, height) {
                triangle_at(canvas, xu, yu);
            } else if is_right(x, y, height) {
                triangle_at(canvas, xu, yu);
            }
        }
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
