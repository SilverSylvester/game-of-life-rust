extern crate rand;

use rand::Rng;
use std::env;
use std::time::Duration;
use std::thread;

fn main() {
    let (w,h) = match (env::args().nth(1), env::args().nth(2)) {
        (Some(w), Some(h)) => match (w.parse(), h.parse()) {
            (Ok(w), Ok(h)) => (w,h),
            _ => err("Couldn't parse arguments."),
        },
        _ => err("Not enough arguments supplied: (width,height)\n\
                  Suggested fullscreen args: cargo run 71 37"),
    };

    let mut rng = rand::thread_rng();
    let mut random_board: Vec<Vec<u8>> = (0..h)
        .map(|_| vec![0; w])
        .collect();

    for x in 0..h {
        for y in 0..w {
            let r = rng.gen::<u8>() % 2;
            random_board[x][y] = r;
        }
    }

    // Main loop
    loop {
        show_board(&random_board);
        update(&mut random_board);
        // Recommended sleep time: 100ms
        thread::sleep(Duration::from_millis(100));
        clear();
    }
}

/// Produces a neighbour count reference board
fn neighbour_count(board: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let rows = board.len();
    let cols = board[0].len();
    let mut neighbours: Vec<Vec<u8>> = (0..rows)
        .map(|_| vec![0; cols])
        .collect();

    for (x,row) in board.iter().enumerate() {
        for (y, _) in row.iter().enumerate() {
            // This workaround is largely to do with the fact
            // Rust only enumerates over a usize type, which proves
            // problematic when you consider '0_usize - 1'.
            let i = x as i16; let j = y as i16;
            let rows = rows as i16; let cols = cols as i16;
            // NOTE: m(a,b) === a `mod` b. This is not the same as
            // '%', which behaves in Rust as it does in C.
            neighbours[x][y] =
                board[m(i - 1, rows)][m(j - 1, cols)] +
                board[x][m(j - 1, cols)] +
                board[m(i + 1, rows)][m(j - 1, cols)] +
                board[m(i - 1, rows)][y] +
                board[m(i + 1, rows)][y] +
                board[m(i - 1, rows)][m(j + 1, cols)] +
                board[x][m(j + 1, cols)] +
                board[m(i + 1, rows)][m(j + 1, cols)]
        }
    }
    neighbours
}

/// Performs a single, in-place board update
fn update(board: &mut Vec<Vec<u8>>) {
    let neighbours = neighbour_count(board);
    let rows = board.len();
    let cols = board[0].len();
    for x in 0..rows {
        for y in 0..cols {
            if board[x][y] == 0 && neighbours[x][y] == 3 {
                board[x][y] = 1;
            }
            else if board[x][y] == 1 
                    && (neighbours[x][y] < 2
                        || neighbours[x][y] > 3) {
                board[x][y] = 0;
            }
        }
    }
}

/// Pretty prints board
fn show_board(board: &Vec<Vec<u8>>){
    print!("-");
    for _ in board[0].iter() { print!("--"); }
    println!("--");
    for (x, row) in board.iter().enumerate() {
        print!("| ");
        for (y, _) in row.iter().enumerate() {
            if board[x][y] == 1 { print!("o "); }
            else { print!("  "); }
        }
        println!("|");
    }
    print!("-");
    for _ in board[0].iter() { print!("--"); }
    println!("--");
}

/// Workaround for the fact that Rust doesn't support
/// modular arithmetic.
fn m(a: i16, b: i16) -> usize {
    (((a % b) + b) % b) as usize
}

/// Clears screen by printing "\ESC[2J" to stdout
fn clear() {
    print!("{}[2J", 27 as char);
}

/// Custom error function that isn't quite as scary
/// as panic!
fn err(msg: &str) -> ! {
    println!("Error: {}", msg);
    std::process::exit(1);
}

