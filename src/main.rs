use rand::Rng;
use std::io;
use std::io::prelude::*;
fn main() {
    let mut grid: [i8; 9];
    let mut turn: i8 = 1;
    loop {
        grid = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        turn %= 2;
        while game_is_over(&grid) == 0 {
            turn += 1;
            print_grid(&grid);
            grid[if turn % 2 == 0 {
                player(&grid)
            } else {
                bot(&grid)
            }] = (turn % 2) - 2;
        }
        print_grid(&grid);
        if game_is_over(&grid) == 1 {
            println!("    Tie.");
        } else {
            if turn % 2 == 0 {
                println!("    Player won.");
            } else {
                println!("    Bot won.");
            }
        }
        pause();
    }
}
fn pause() {
    let mut stdout = io::stdout();
    // The cursor will stay at the end of the line, so print without a newline and flush manually
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();
    // Read a single byte and discard
    io::stdin().read(&mut [0u8]).unwrap();
}
fn clear_screen() {
    /* Clear the screen */
    print!("\x1B[2J\x1B[1;1H");
}
fn print_grid(grid: &[i8; 9]) {
    /* Print grid passed as parameter, use a specific order to print the same pattern as the numpad */
    clear_screen();
    println!("   ╔═══════════════════╗\n   ║    Tic Tac Toe    ║\n   ╚═══════════════════╝");
    println!();
    for i in [6, 3, 0].iter() {
        print!("\t ");
        print_grid_cell(grid[*i]);
        print!(" ║ ");
        print_grid_cell(grid[*i + 1]);
        print!(" ║ ");
        print_grid_cell(grid[*i + 2]);
        println!();
        if *i > 0 {
            println!("\t═══╬═══╬═══");
        }
    }
    println!();
}
fn print_grid_cell(cell: i8) {
    /* Print the cell's content corresponding to its value */
    match cell {
        -1 => print!("O"),
        -2 => print!("X"),
        _ => print!("{}", cell),
    }
}
fn game_is_over(grid: &[i8; 9]) -> u8 {
    /* Check if the grid is full or if a player won (0: nothing, 1: grid is full, 2: a player won ) */
    let mut r: u8 = 1;
    // check if the grid is full
    for c in grid {
        if *c > 0 {
            r = 0;
        }
    }
    // check if a player won
    for i in [0, 1, 2].iter() {
        if grid[*i] == grid[*i + 3] && grid[*i] == grid[*i + 6] {
            r = 2;
        }
    }
    for i in [0, 3, 6].iter() {
        if grid[*i] == grid[*i + 1] && grid[*i] == grid[*i + 2] {
            r = 2;
        }
    }
    if grid[0] == grid[4] && grid[0] == grid[8] {
        r = 2;
    }
    if grid[2] == grid[4] && grid[2] == grid[6] {
        r = 2;
    }
    r
}
fn check_valid_move(m: usize, grid: &[i8; 9]) -> bool {
    /* Check if the move tested is valid or not */
    grid[m] > 0
}
fn player(grid: &[i8; 9]) -> usize {
    /* Ask user to play */
    let mut t: usize = 10;
    let mut b: bool = false;
    let mut s: String;
    let mut c: u8;
    while !b {
        t = 10;
        while t == 10 {
            println!("Enter a number [1-9]:");
            while {
                s = String::new();
                io::stdin().read_line(&mut s).expect("Failed to read line");
                match s.trim().parse::<u8>() {
                    Ok(_) => false,
                    Err(_) => true,
                }
            } { }
            c = s.trim().parse().expect("Please type a number!");
            t = if c != 0 && c <= 9 { c as usize } else { t };
        }
        b = check_valid_move(t - 1, &grid);
    }
    t - 1
}
fn bot(grid: &[i8; 9]) -> usize {
    /* Simulate bot play */
    let mut t: usize = 10;
    let mut b: bool = false;
    while !b {
        t = rand::thread_rng().gen_range(0..9) as usize;
        b = check_valid_move(t, &grid);
    }
    t
}
