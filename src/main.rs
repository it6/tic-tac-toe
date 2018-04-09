use std::io;

fn main() {
    let mut player = true;
    let mut board = [0, 0, 0, 0, 0, 0, 0, 0, 0];
    loop {
        if player {
            println!("Player 1 turn : ");
        } else {
            println!("Player 2 turn : ");
        }
        let mut num = String::new();
        io::stdin()
            .read_line(&mut num)
            .ok()
            .expect("Couldn't read line");
        match num.trim().parse::<usize>() {
            Ok(i @ 1...9) => update_board(i, &mut board, &mut player),
            Ok(i) => invalid_number(i),
            Err(e) => println!("not an integer: {}", e),
        };
        let winner = calculate_winner(&mut board);
        if winner.0 {
            println!("{}", winner.1);
            break;
        }
    }
}

fn calculate_winner(board: &mut [u32; 9]) -> (bool, String) {
    let mut ans = (false, String::from("none"));
    let players = (1, 2);
    let winning_combinations = [
        (0, 1, 2),
        (3, 4, 5),
        (6, 7, 8),
        (0, 3, 6),
        (1, 4, 7),
        (2, 5, 8),
        (0, 4, 8),
        (2, 4, 6),
    ];

    for values in winning_combinations.iter() {
        if players.0 == board[values.0] && players.0 == board[values.1]
            && players.0 == board[values.2]
        {
            ans.0 = true;
            ans.1 = String::from("player 1 wins")
        }
        if players.1 == board[values.0] && players.1 == board[values.1]
            && players.1 == board[values.2]
        {
            ans.0 = true;
            ans.1 = String::from("player 2 wins")
        }
        if game_complete(&board) {
            ans.0 = true;
            ans.1 = String::from("game draw");
        }
    }
    return ans;
}

fn game_complete(board: &[u32; 9]) -> bool {
    for val in board.iter() {
        if *val < 1 {
            return false;
        }
    }
    true
}

fn invalid_number(i: usize) {
    println!("invalid number: {}", i);
    println!("enter a number between 1-9",);
}

fn update_board(number: usize, board: &mut [u32; 9], player: &mut bool) {
    if board[number - 1] == 0 {
        if *player {
            board[number - 1] = 1;
            *player = false;
        } else {
            board[number - 1] = 2;
            *player = true;
        }
        println!("******************************************");
        println!(
            "\n {}-{}-{} \n {}-{}-{} \n {}-{}-{} \n",
            board[0],
            board[1],
            board[2],
            board[3],
            board[4],
            board[5],
            board[6],
            board[7],
            board[8]
        );
        println!("******************************************");
    } else {
        println!("already picked, pick a valid number");
        println!("\n");
    }
}
