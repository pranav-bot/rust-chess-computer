//DONE: A way to represnt the state of the game (piece posiotions, whose turn, who can castle ...) (FEN Strings)
//TODO: A way to generate legal moves
//TODO: A way to search legal moves
//TODO: A way to select the "best" moves
mod position;
mod utils;
mod rayattacks;
mod knightattacks;
mod pawnattacks;
mod movegeneration;
use knightattacks::KnightAttacks;
use position::*;

#[derive(Debug, Clone)]
pub struct Game {
    position: Position,
    knight_attacks: KnightAttacks
}

impl Game {
    fn new() -> Self {
        Self {
            position: Position::new(),
            knight_attacks: KnightAttacks::new()
        }
    }
    fn read_fen(fen: &str) -> Self {
        Self {
            position: Position::read_fen(fen),
            knight_attacks: KnightAttacks::new()
        }
    }
}

fn main() {
    let fen_string = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let not_alot = "5k2/8/8/4N3/8/8/8/5K2 w - - 0 1";
    let not_alot2 = "5k2/8/8/4N3/2N5/8/8/5K2 w - - 0 1";
    let game = Position::read_fen(&not_alot2);
    println!("{}", game.to_string());
    println!("{:?} {:?} {}", game.active_color, game.en_passant, game.fullmove_clock);
}


