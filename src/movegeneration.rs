use crate::{utils::{_bit_scan, extract_bits}, Color, Game, Piece, PieceType, Position};

fn generate_moves(game: &Game) -> Vec<Position> {
    let mut new_positions = vec![];
    for piece in &game.position.pieces {
        if piece.color == game.position.active_color {
            match &piece.piece_type {
                PieceType::Knight => {
               let positions = generate_knight_moves(&piece, &game);
               new_positions.extend(positions);
                },
                _ => panic!("Piece type not yet supported")
            }
        }
    }
    new_positions
}

fn generate_knight_moves(piece: &Piece, game: &Game) -> Vec<Position> {
    let mut attacks = game.knight_attacks.0[_bit_scan(piece.position)];
    let position = &game.position;
    
    let own_occupancy = match piece.color {
        Color::White => position.white_occupancy,
        Color::Black => position.black_occupancy
    };
    attacks &= !own_occupancy;
    let potential_moves = extract_bits(attacks);
    let mut new_positions = vec![];
    for pmove in potential_moves {
        let mut new_position = (*game).position.clone();
        new_position.move_piece(piece.position, pmove);
        new_positions.push(new_position);

    } 

    new_positions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_knight_moves_test() {
        let not_alot = "8/8/8/4N3/2N5/8/8/8 w - - 0 1";
        let game = Game::read_fen(&not_alot);
        println!("{}", game.position.to_string());
        let moves = generate_knight_moves(&game.position.pieces[0], &game);
        let new_positions = [19,21, 30, 42, 46, 51, 53];
        assert_eq!(moves.len(), 7);
        for v in moves {
            assert_eq!(v.pieces.len(), 2);
            let piece = &v.pieces[0];
            let index = _bit_scan(piece.position);
            assert!(new_positions.contains(&index));
        }
    }
    #[test]
    fn generate_three_knight_moves_test() {
        let not_alot = "8/5N2/8/4N3/2N5/8/8/8 w - - 0 1";
        let game = Game::read_fen(&not_alot);
        println!("{}", game.position.to_string());
        let moves = generate_knight_moves(&game.position.pieces[1], &game);
        let new_positions = [19,21, 30, 42, 46, 51];
        assert_eq!(moves.len(), 6);
        for v in moves {
            assert_eq!(v.pieces.len(), 3);
            let piece = &v.pieces[1];
            let index = _bit_scan(piece.position);
            assert!(new_positions.contains(&index));
        }
    }
    #[test]
    fn generate_three_knight_moves_one_black_test() {
        let not_alot = "8/5n2/8/4N3/2N5/8/8/8 w - - 0 1";
        let game = Game::read_fen(&not_alot);
        println!("{}", game.position.to_string());
        let moves = generate_knight_moves(&game.position.pieces[1], &game);
        let new_positions = [19,21, 30, 42, 46, 51,53];
        assert_eq!(moves.len(), 7);
        for v in moves {
            let piece;
           if v.pieces.len() ==3 {
            piece = &v.pieces[1];
           } else if v.pieces.len() ==2 {
               piece = &v.pieces[0];
           } else {
            panic!("Invalid")
           }
           assert_eq!(piece.color, Color::White);
           let index = _bit_scan(piece.position);
           assert!(new_positions.contains(&index));
        }
    }
    #[test]
    fn generate_three_knight_moves_two_black_test() {
        let not_alot = "8/5n2/8/4N3/2n5/8/8/8 w - - 0 1";
        let game = Game::read_fen(&not_alot);
        println!("{}", game.position.to_string());

        for piece in &game.position.pieces {
            println!("{:?}", piece)
        }

        let moves = generate_knight_moves(&game.position.pieces[1], &game);
        let new_positions = [19, 21, 26, 30, 42, 46, 51, 53];
        assert_eq!(moves.len(), 8);
        for v in moves {
            let mut piece = &v.pieces[0];
           for pie in &v.pieces {
            if pie.color == Color::White {
                piece = pie;
                break;
            }
           }
           assert!(v.pieces.len()==2 || v.pieces.len()==3);
           let index = _bit_scan(piece.position);
           assert!(new_positions.contains(&index));
        }
    }   
}