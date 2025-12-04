use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

#[derive(Debug, PartialEq)]
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

// Circle dial of the safe clicks with a range 0..90, 50 is initial position
fn main() {
    let file = match File::open("in.txt") {
        Ok(f) => f,
        Err(_) => panic!("Cannot read input"), // todo: find better way how to handle Result err
    };

    let res = solve(file);
    match res {
        Ok(x) => println!("ans: {x}"),
        Err(e) => eprintln!("Unexpected error: {}", e),
    }
}

fn solve<R: Read>(reader: R) -> Result<i32, String> {
    let res = load_rotations(reader);

    res.map(|rotations| do_solve(rotations))
        .map_err(|e| format!("Unexpected error: {}", e))
}

fn do_solve(rotations: Vec<Rotation>) -> i32 {
    let mut curr: i32 = 50;
    let mut ans = 0;
    let len = 100;
    
    for r in rotations {        
        let step = match r.dir {
            Direction::Left => -r.clicks,
            Direction::Right => r.clicks,            
        };
        
        curr = (curr + (step % len) + len) % len;
        if curr == 0 {
            ans += 1;
        }
    }

    ans
}

fn load_rotations<R: Read>(reader: R) -> io::Result<Vec<Rotation>> {
    let mut reader = BufReader::new(reader);

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

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn should_process_simple_input_successfuly() {        
        let input = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

        let cursor = Cursor::new(input.as_bytes());
        let res = solve(cursor);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 3);
    }

    #[test]    
    fn should_process_overflow_successfuly() {        
        let input = "\
L1000
L50
R1000
R50
";

        let cursor = Cursor::new(input.as_bytes());
        let res = solve(cursor);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 2);
    }
}
