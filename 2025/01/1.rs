use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Rotation {
    dir: Direction,
    clicks: i32,
}

// dial safe clicks range is 0..90, 50 is initial
fn main() {
    let res = load_rotations("in.txt");
    match res {
        Ok(rotations) => {
            println!("{:#?}", rotations);
        }
        Err(e) => eprintln!("Unexpected error: {}", e),
    }    
}

fn load_rotations(filepath: &str) -> io::Result<Vec<Rotation>> {
    let file = File::open(filepath)?;
    let mut reader = BufReader::new(file);

    let mut rotations: Vec<Rotation> = Vec::new();
    let mut buf = String::new();

    while reader.read_line(&mut buf)? != 0 {
        let dir = match buf.chars().next() {
            Some('L') => Direction::Left,
            Some('R') => Direction::Right,
            _ => unreachable!("Unexpected direction"),
        };

        let clicks = buf[1..]
            .trim()
            .parse::<i32>()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        buf.clear();

        rotations.push(Rotation { dir, clicks });
    }

    Ok(rotations)
}
