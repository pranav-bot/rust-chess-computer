use crate::{utils::{set_bit, BitBoard}, Color};

pub struct PawnAttacks {
    white_forward_moves: Vec<BitBoard>,
    white_diagonal_moves: Vec<BitBoard>,
    black_forward_moves: Vec<BitBoard>,
    black_diagonal_moves: Vec<BitBoard>
}

impl PawnAttacks {
    fn initialize() -> Self {
        let mut w_forward = vec![];
        let mut w_diagonal = vec![];
        let mut b_forward = vec![];
        let mut b_diagonal = vec![];

        for row in 1..=8 {
            for col in 1..=8 {
                let f = forward_move(row,col, Color::White);
                let d = diagonal_move(row, col, Color::White);
                w_forward.push(f);
                w_diagonal.push(d);
                let f = forward_move(row,col, Color::Black);
                let d = diagonal_move(row, col, Color::Black);
                b_forward.push(f);
                b_diagonal.push(d);
            }
        }
        Self { 
            white_forward_moves: w_forward,
            white_diagonal_moves: w_diagonal,
            black_forward_moves: b_forward,
            black_diagonal_moves: b_diagonal
         }
    }
}

//00000000
//00000000
//00000000
//00000000
//00000000
//00000000
//00000000
//00000000

fn forward_move(row: i32, col: i32, color: Color) -> BitBoard {
    if row==1 || row==8 {
        return 0;
    }
    let mut bitboard = 0;
    if color == Color::White {
        if row<8 {
            bitboard |= set_bit(row+1, col);
        }
        if row==2 {
            bitboard |= set_bit(row+1, col);
            bitboard |= set_bit(row+2, col);
        }
    }
    else {
        if row>1 {
            bitboard |= set_bit(row-1, col);
        }
        if row==7 {
            bitboard |= set_bit(row-1, col);
        }
    }
    bitboard
}

fn diagonal_move(row: i32, col: i32, color: Color) -> BitBoard {
    if row==1 || row==8 {
        return 0;
    }
    let mut bitboard = 0;
    if color==Color::White {
        if row<8{
            bitboard |= set_bit(row+1, col+1);
            bitboard |= set_bit(row+1, col-1);
        }
    }
    else{
        if row<8{
            bitboard |= set_bit(row-1, col+1);
            bitboard |= set_bit(row-1, col-1);
        }
    }
    bitboard
}

#[cfg(test)]
mod tests {
    use crate::utils::{_bit_scan, _bit_scan_backwards};

    use super::*;

    #[test]
    fn test_seccond_row_white_pawn() {
        let row = 2;
            for col in 1..=8 {
                let bitboard = forward_move(row, col, Color::White);
                let lsb = _bit_scan(bitboard);
                let msb = _bit_scan_backwards(bitboard);
                //print_bitboard(bitboard, Some(8))
                let expected_lsb = (col -1) + (row+1-1) * 8;
                let expected_msb = (col -1) + (row+2-1) * 8;
                assert_eq!(lsb, expected_lsb as usize);
                assert_eq!(msb, expected_msb as usize)
            }
        }

        #[test]
        fn test_seccond_row_black_pawn() {
            let row = 2;
                for col in 1..=8 {
                    let bitboard = forward_move(row, col, Color::Black);
                    let lsb = _bit_scan(bitboard);
                    //print_bitboard(bitboard, Some(8))
                    let expected_lsb = (col -1) + (row-1-1) * 8;
                    assert_eq!(lsb, expected_lsb as usize);

                }
            }

        #[test]
        fn test_middle_rows_white_pawn() {
            for row in 3..=7{
                for col in 1..=8 {
                    let bitboard = forward_move(row, col, Color::White);
                    let lsb = _bit_scan(bitboard);
                    let expected_lsb = (col -1) + (row+1-1) * 8;
                    assert_eq!(lsb, expected_lsb as usize);
            }
        }
    }

        #[test]
        fn test_middle_rows_black_pawn() {
            for row in 3..=7 {
                for col in 1..=8 {
                    let bitboard = forward_move(row, col, Color::Black);
                    let lsb = _bit_scan(bitboard);
                    let expected_lsb = (col -1) + (row-1-1) * 8;
                    assert_eq!(lsb, expected_lsb as usize);
                }
            }
        }

        #[test]
        fn test_edges() {
            for color in [Color::White, Color::White] {
            for row in [1, 8] {
                for col in 1..=8 {
                    let bitboard = forward_move(row, col, color);
                    assert_eq!(bitboard, 0);
                }
            }
            }
        }
    }
