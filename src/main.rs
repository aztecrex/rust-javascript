// use std::os::raw::c_char;

// extern {
//     pub fn emscripten_asm_const(s: *const c_char);
// }

// fn main() {
//     let code : &'static [u8] = b"console.log('Hello Rust!');\0";

//     unsafe {
//         emscripten_asm_const(&code[0] as *const c_char);
//     }
// }

#[derive(Debug)]
enum Direction { North, South, East, West }

fn is_north(dir: Direction) -> bool {
    match dir {
        Direction::North => true,
        _ => false,
    }
}

fn main() {
    let points = Direction::South;
    println!("{:?}", points);
    let compass = is_north(points);
    println!("{}", compass);
}
