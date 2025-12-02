use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[allow(dead_code)]
enum Direction {
    Left,
    Right,
}

#[allow(dead_code)]
struct Rotation {
    dir: Direction,
    clicks: i32,
}

// dial safe clicks range is 0..90, 50 is initial
fn main() {
    println!("hello");
    let _ = load_rotations("in.txt");
}

// fn load_rotations(filepath: String) -> io::Result<Vec<Rotation>> {
fn load_rotations(filepath: &str) -> io::Result<()> {
    let file = File::open(filepath)?;
    let mut reader = BufReader::new(file);

    let mut buf = String::new();

    while reader.read_line(&mut buf)? != 0 {
        println!("{}", buf);
        buf.clear();
    }

    Ok(())
}
