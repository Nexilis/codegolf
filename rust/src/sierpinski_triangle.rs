// clear && rustfmt sierpinski_triangle.rs && rustc sierpinski_triangle.rs && ./sierpinski_triangle

// my best code.golf solution for now:
// pub fn main(){for i in vec![15,-1,14,1,-1,13,3,-1,12,1,1,1,-1,11,7,-1,10,1,5,1,-1,9,3,3,3,-1,8,1,1,1,1,1,1,1,-1,7,15,-1,6,1,13,1,-1,5,3,11,3,-1,4,1,1,1,9,1,1,1,-1,3,7,7,7,-1,2,1,5,1,5,1,5,1,-1,1,3,3,3,3,3,3,3,-1,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1]{if i==-1{println!()}else{for _ in 0..i{print!(" ")}print!("▲")}}}

pub fn main() {
    let base: i32 = 2;
    let width = (base.pow(5) - 1) as usize; // 31
    let height = base.pow(4) as usize; // 16

    let mut canvas = vec![vec![' '; width]; height];
    let start_row = &mut canvas[0];
    draw_triangle_at(start_row, height - 1);
    draw_next_row_rec(&mut canvas, 0);

    for y in canvas {
        println!("{:?}", y);
    }
}

fn draw_next_row_rec(canvas: &mut Vec<Vec<char>>, pos_y: usize) {
    if pos_y >= 15 {
        return;
    }
    let current_row = &canvas[pos_y];
    let mut result = vec![];
    for (pos, e) in current_row.iter().enumerate() {
        if *e == '▲' {
            let pos_left = if pos >= 2 { pos - 2 } else { 0 };
            let two_left = canvas[pos_y][pos_left];
            if two_left == ' ' {
                result.push(pos - 1);
            }

            let pos_right = if pos < 29 { pos + 2 } else { 30 };
            let two_right = canvas[pos_y][pos_right];

            if two_right == ' ' {
                result.push(pos + 1);
            }
        }
    }
    let next_row = &mut canvas[pos_y + 1];
    for elem in result {
        draw_triangle_at(next_row, elem);
    }
    draw_next_row_rec(canvas, pos_y + 1);
}

fn draw_triangle_at(row: &mut Vec<char>, x: usize) {
    row[x] = '▲';
}
