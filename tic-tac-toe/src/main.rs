use rand::Rng;
use std::io;

fn print_board(board: &[char]) {
    println!("");
    println!(" {} | {} | {} ", board[0], board[1], board[2]);
    println!("---+---+---");
    println!(" {} | {} | {} ", board[3], board[4], board[5]);
    println!("---+---+---");
    println!(" {} | {} | {} ", board[6], board[7], board[8]);
    println!("");
}

fn check_win(board: &[char], player: char) -> bool {
    let wins = [
        [0,1,2], [3,4,5], [6,7,8], // rows
        [0,3,6], [1,4,7], [2,5,8], // cols
        [0,4,8], [2,4,6],          // diagonals
    ];

    for combo in wins {
        if board[combo[0]] == player &&
           board[combo[1]] == player &&
           board[combo[2]] == player 
        {
            return true;
        }
    }
    false
}

fn board_full(board: &[char]) -> bool {
    !board.contains(&' ')
}

fn main() {
    let mut board = vec![' '; 9]; //generates the board here
    let mut rng = rand::thread_rng();

    // 0 = user starts, 1 = computer starts
    let turn = rng.gen_range(0..=1);
    let mut user_turn = turn == 0;

    println!("Tic Tac Toe!");
    if user_turn {
        println!("You go first (X)!");
    } else {
        println!("Computer goes first (O)!");
    }

    loop {
        print_board(&board);

        if user_turn {
            // USER TURN
            println!("Enter a position (1-9):");

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            let pos: usize = match input.trim().parse() {
                Ok(n) => n,
                Err(_) => {
                    println!("Invalid number! Try again.");
                    continue;
                }
            };

            if pos < 1 || pos > 9 {
                println!("Position must be 1-9!");
                continue;
            }

            let idx = pos - 1;

            if board[idx] != ' ' {
                println!("Position already taken try another!");
                continue;
            }

            board[idx] = 'X';

            if check_win(&board, 'X') {
                print_board(&board);
                println!("YOU WIN");
                return;
            }

        } else {
            // COMPUTER TURN
            println!("Computer thinking...");

            loop {
                let idx = rng.gen_range(0..9);
                if board[idx] == ' ' {
                    board[idx] = 'O';
                    break;
                }
            }

            if check_win(&board, 'O') {
                print_board(&board);
                println!("Computer wins!\nYou just lost to a computer");
                return;
            }
        }

        if board_full(&board) {
            print_board(&board);
            println!("It's a TIE ");
            return;
        }

        user_turn = !user_turn;
    }
}
