use crate::{game, utils::{_bit_scan, print_bitboard}, Game, Piece, PieceType};

fn generate_moves(game: &Game) -> Vec<Game> {
    let mut new_positions = vec![];
    for piece in &game.pieces {
        if piece.color == game.active_color {
            match piece.piece_type {
                PieceType::King => {
               let positions = generate_knight_moves(&piece, &game);
               new_positions.extend(positions);
                },
                typ => panic!("Piece type not yet supported")
            }
        }
    }
    new_positions
}

fn generate_knight_moves(piece: &Piece, game: &Game) -> Vec<Game> {
    let attacks = game.knight_attacks.0[_bit_scan(piece.position)];
    print_bitboard(attacks, None);
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_knight_moves_test() {
        let not_alot = "5k2/8/8/4N3/8/8/8/5K2 w - - 0 1";
        let game = Game::read_fen(&not_alot);
        let moves = generate_knight_moves(&game.pieces[0], &game);
        println!("{:?}", moves);
    }
}