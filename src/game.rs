use std::io;
use std::collections::HashSet;
use rand;
use rand::random;


type Board = Vec<Vec<String>>;

enum Turn {
    Player, 
    Bot,
}

pub struct Game {
    board: Board,
    current_turn: Turn,
    bot_moves: HashSet<u32>,
    player_moves: HashSet<u32>,
}

impl Game{
    pub fn new() -> Game{
        let first_row: Vec<String> = vec![
            String::from("1"),
            String::from("2"),
            String::from("3")
        ];

        let second_row: Vec<String> = vec![
            String::from("4"),
            String::from("5"),
            String::from("6")
        ];

        let third_row: Vec<String> = vec![
            String::from("7"),
            String::from("8"),
            String::from("9")
        ];
        
        Game {
            board: vec![first_row, second_row, third_row],
            current_turn: Self::pick_player(),
            bot_moves: HashSet::new(),
            player_moves: HashSet::new()
        }
    }

    pub fn play_game(&mut self){
        let mut finished: bool = false;

        while !finished{
            self.play_turn();

            if self.game_is_won(){
                self.print_board();

                match self.current_turn {
                    Turn::Player => println!("You won!"),
                    Turn::Bot => println!("You lost!")
                };

                finished = Self::player_is_finished();

                self.reset();
                self.current_turn = Turn::Player;
            }
            else{
                if self.game_is_finished(){
                    if self.game_is_won(){
                        self.print_board();

                        match self.current_turn {
                            Turn::Player => println!("You won!"),
                            Turn::Bot => println!("You lost!")
                        };
                    }else{
                        self.print_board();
                        println!("Game ended with a draw!");
                    }

                    finished = Self::player_is_finished();

                    self.reset();
                }
                else{
                    self.current_turn = self.get_next_turn();
                }
            }
        }

        println!("Bye");
    }

    ///////////////////////
    //  Private methods
    ///////////////////////

    fn play_turn(&mut self){
        self.print_board();

        let (token, valid_move) = match self.current_turn {
            Turn::Player => (String::from("X"), self.get_player_move()),
            Turn::Bot => (String::from("O"), self.get_bot_move())
        };

        let (row, col) = Self::get_board_location(valid_move);

        self.board[row][col] = token;
    }

    fn print_board(&mut self){
        let separator = "+---+---+---+";

        println!("\n{}", separator);

        for row in &self.board{
            println!("| {} |\n{}", row.join(" | "), separator);
        }

        print!("\n");
    }

    fn get_player_move(&mut self) -> u32 {
        loop {
            let mut player_input = String::new();

            println!("\nYour turn, please enter your move (an integer between 1 and 9): ");

            match io::stdin().read_line(&mut player_input){
                Err(_) => println!("Error reading input, try again!"),
                Ok(_) => match self.validate(&player_input){
                    Err(err) => print!("{}", err),
                    Ok(num) => {
                        self.player_moves.insert(num);
                        return num
                    }
                },
            }
        }
    }

    fn validate(&self, input: &str) -> Result<u32, String>{
        // the turbofish, ::<type>
        match input.trim().parse::<u32>(){
            Err(_)=> Err(String::from("Please input a valid unsigned integer!")),
            Ok(number)=> {
                if self.is_valid_move(number){
                    Ok(number)
                }else{
                    Err(String::from("Please input a number, between 1 and 9, not already chosen!"))
                }
            }
        }
    }

    fn is_valid_move(&self, unchecked_move: u32)->bool{
        match unchecked_move {
            1..=9 => {
                let (row, col) = Self::get_board_location(unchecked_move);

                match self.board[row][col].as_str(){
                    "X" | "O" => false,
                    _ => true,
                }
            }
            _ => false,
        }
    }

    /// Try to find the move that will make the bot win the game.
    fn find_bot_winning_move(&self) -> Option<u32> {
        for i in 1..=9{

            if !self.bot_moves.contains(&i) && !self.player_moves.contains(&i){
                let r = (i-1)/3;
                let c = (i-1)%3;

                let is_winning_move =
                    (r==0 && self.bot_moves.contains(&(i+3)) && self.bot_moves.contains(&(i+6))) ||
                    (r == 1 && self.bot_moves.contains(&(i-3)) && self.bot_moves.contains(&(i+3))) ||
                    (r == 2 && self.bot_moves.contains(&(i-3)) && self.bot_moves.contains(&(i-6))) ||
                    (c == 0 && self.bot_moves.contains(&(i+1)) && self.bot_moves.contains(&(i+2))) ||
                    (c == 1 && self.bot_moves.contains(&(i-1)) && self.bot_moves.contains(&(i+1))) ||
                    (c == 2 && self.bot_moves.contains(&(i-1)) && self.bot_moves.contains(&(i-2))) ||
                    (r == c &&
                        ((c == 0 && self.bot_moves.contains(&(i+4)) && self.bot_moves.contains(&(i+8))) ||
                            (c == 1 && ((self.bot_moves.contains(&(i-4)) && self.bot_moves.contains(&(i+4))) ||
                                (self.bot_moves.contains(&(i-2)) && self.bot_moves.contains(&(i+2))))) ||
                            (c == 2 && self.bot_moves.contains(&(i-4)) && self.bot_moves.contains(&(i-8))))) ||
                    ((r as i32 - c as i32).abs() == 2 && (
                        (r==0 && self.bot_moves.contains(&(i+2)) && self.bot_moves.contains(&(i+4))) ||
                        (c==0 && self.bot_moves.contains(&(i-2)) && self.bot_moves.contains(&(i-4)))))

                    ;

                if is_winning_move {
                    Some(i);
                }
            }
        }

        return None;
    }

    /// Try to find the move that will not make the bot lose the game.
    fn find_bot_saving_move(&self) -> Option<u32> {
        for i in 1..=9{

            if !self.bot_moves.contains(&i) && !self.player_moves.contains(&i){
                let r = (i-1)/3;
                let c = (i-1)%3;

                let is_saving_move =
                    (r == 0 && self.player_moves.contains(&(i+3)) && self.player_moves.contains(&(i+6))) ||
                    (r == 1 && self.player_moves.contains(&(i-3)) && self.player_moves.contains(&(i+3))) ||
                    (r == 2 && self.player_moves.contains(&(i-3)) && self.player_moves.contains(&(i-6))) ||
                    (c == 0 && self.player_moves.contains(&(i+1)) && self.player_moves.contains(&(i+2))) ||
                    (c == 1 && self.player_moves.contains(&(i-1)) && self.player_moves.contains(&(i+1))) ||
                    (c == 2 && self.player_moves.contains(&(i-1)) && self.player_moves.contains(&(i-2))) ||
                    (r == c && (
                        (c == 0 && self.player_moves.contains(&(i+4)) && self.player_moves.contains(&(i+8))) ||
                        (c == 1 && (
                                (self.player_moves.contains(&(i-4)) && self.player_moves.contains(&(i+4))) ||
                                (self.player_moves.contains(&(i-2)) && self.player_moves.contains(&(i+2)))
                            )) ||
                        (c == 2 && self.player_moves.contains(&(i-4)) && self.player_moves.contains(&(i-8)))
                    )) ||
                    ((r as i32 - c as i32).abs() == 2 && (
                        (r==0 && self.player_moves.contains(&(i+2)) && self.player_moves.contains(&(i+4))) ||
                        (c==0 && self.player_moves.contains(&(i-2)) && self.player_moves.contains(&(i-4)))
                    ))
                ;

                if is_saving_move {
                    return Some(i);
                }
            }
        }

        return None;
    }

    fn get_bot_move(&mut self) -> u32 {

        let mut bot_move: u32;

        if let Some(winning_move) = self.find_bot_winning_move() {
            bot_move = winning_move;
        }
        else if let Some(saving_move) = self.find_bot_saving_move(){
            bot_move = saving_move;
        }
        else {
            bot_move = rand::random::<u32>() % 9 + 1;

            while !self.is_valid_move(bot_move) {
                bot_move = rand::random::<u32>() % 9 + 1;
            }
        }

        self.bot_moves.insert(bot_move);

        println!("Bot's turn, it played: {}", bot_move);

        bot_move

    }

    fn game_is_won(&self)->bool{
        let mut all_same_row = false;
        let mut all_same_col = false;

        for index in 0..3 {
            all_same_row |= self.board[index][0] == self.board[index][1] && self.board[index][1] == self.board[index][2];
            all_same_col |= self.board[0][index] == self.board[1][index] && self.board[1][index] == self.board[2][index];
        }

        let all_same_diag_1 = self.board[0][0] == self.board[1][1] && self.board[1][1] == self.board[2][2];
        let all_same_diag_2 = self.board[0][2] == self.board[1][1] && self.board[1][1] == self.board[2][0];
    
        all_same_row || all_same_col || all_same_diag_1 || all_same_diag_2
    }

    fn game_is_finished(&self)->bool{
        9 == self.player_moves.len() + self.bot_moves.len()
    }

    fn reset(&mut self){
        self.current_turn = Self::pick_player();
        self.board = vec![
            vec![String::from("1"), String::from("2"), String::from("3")],
            vec![String::from("4"), String::from("5"), String::from("6")],
            vec![String::from("7"), String::from("8"), String::from("9")]
        ];
        self.bot_moves.clear();
        self.player_moves.clear();
    }

    fn get_next_turn(&self) -> Turn {
        match self.current_turn {
            Turn::Player => Turn::Bot,
            Turn::Bot => Turn::Player,
        }
    }

    //////////////////////////
    //  Static methods
    //////////////////////////
    fn pick_player()->Turn{
        let n = random::<u32>() % 2 + 1;

        match n {
            1 => Turn::Player,
            _ => Turn::Bot
        }
    }

    fn get_board_location(game_move: u32) -> (usize, usize){
        let row = (game_move - 1) / 3;
        let col = (game_move - 1) % 3;

        (row as usize, col as usize)
    }

    fn player_is_finished() -> bool {
        let mut player_input = String::new();
        println!("Are you finished playing (y/n)?: ");

        match io::stdin().read_line(&mut player_input){
            Ok(_)=>{
                let temp = player_input.to_lowercase();
                temp.trim() == "y" || temp.trim() == "yes"
            }
            Err(_)=>false,
        }
    }
}


