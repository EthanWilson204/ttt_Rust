use std::io::{self, Write};

//display an empty board
fn print_board(board: &[[char; 3]; 3]) {
    for row in board {
        println!(" {} | {} | {}", row[0], row[1], row[2]);
        println!("---|---|---");
    }
}

fn check_win(board: &[[char; 3]; 3], player: char) -> bool {
    // Check rows and columns
    for i in 0..3 {
        if (board[i][0] == player && board[i][1] == player && board[i][2] == player)
            || (board[0][i] == player && board[1][i] == player && board[2][i] == player)
        {
            return true;
        }
    }

    // Check diagonals
    if (board[0][0] == player && board[1][1] == player && board[2][2] == player)
        || (board[0][2] == player && board[1][1] == player && board[2][0] == player)
    {
        return true;
    }

    false
}

fn main() {
    println!("Welcome to Tic-Tac-Toe!");

    let mut board = [[' '; 3]; 3];
    let mut current_player = 'X';

    loop {
        println!("\nCurrent board:");
        print_board(&board);

        println!("\nPlayer {}, enter your move (row col):", current_player);
        io::stdout().flush().expect("Failed to flush stdout.");

        //create a new string to make a move. Check to see if the line reads
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line.");

        let coords: Vec<usize> = input
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        //check to see if board placement is valid
        if coords.len() != 2 || coords[0] > 2 || coords[1] > 2 {
            println!("Invalid input. Please enter two numbers between 0 and 2.");
            continue;
        }

        let row = coords[0];
        let col = coords[1];

        //check to see if the board selection is taken
        if board[row][col] != ' ' {
            println!("That cell is already occupied. Please choose another one.");
            continue;
        }

        board[row][col] = current_player;

        //every turn, check to see based on the board if a person has won/which person has won
        if check_win(&board, current_player) {
            println!("\nCurrent board:");
            print_board(&board);
            println!("Player {} wins!", current_player);
            break;
        }

        //check to see if all the spaces are filled
        if board.iter().all(|row| row.iter().all(|&cell| cell != ' ')) {
            println!("\nCurrent board:");
            print_board(&board);
            println!("It's a draw!");
            break;
        }

        //swap players between turns
        current_player = if current_player == 'X' { 'O' } else { 'X' };
    }
}
