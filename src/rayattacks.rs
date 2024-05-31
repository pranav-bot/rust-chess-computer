type BitBoard = u64;

pub struct Rays {
    n_rays: Vec<BitBoard>,
    e_rays: Vec<BitBoard>,
    nw_rays: Vec<BitBoard>,
    ne_rays: Vec<BitBoard>,
    w_rays: Vec<BitBoard>,
    s_rays: Vec<BitBoard>,
    sw_rays: Vec<BitBoard>,
    se_rays: Vec<BitBoard>,
}

macro_rules! make_rays {
    ($ray_fn:ident) => {{
        let mut rays = vec![];
        for row in 1..=8 {
            for col in 1..=8 {
                rays.push($ray_fn(row, col));
            }
        }
        rays
    }};
}

fn set_bit(bitboard: BitBoard, row_col: (i64, i64)) -> BitBoard {
    let row = row_col.0;
    let col = row_col.1;
    if row < 1 || row >8 || col < 1 || col >8 {
        return bitboard;
    } 
    bitboard | (1<<((col-1)+(row-1)*8))
}

macro_rules! define_ray {
    ($name:ident, $offset_fun:expr) => {
        fn $name(row: i64, col: i64) -> BitBoard {
            let mut bitboard = 0;
        
            for offset in 1..=8 {
                bitboard = set_bit(bitboard, $offset_fun(row, col, offset));
            }
            bitboard
        }
    };
}

define_ray!(n_ray, |row, col, offset| (row+offset, col));
define_ray!(e_ray, |row, col, offset| (row, col+offset));
define_ray!(nw_ray, |row, col, offset| (row+offset, col-offset));
define_ray!(ne_ray, |row, col, offset| (row+offset, col+offset));
define_ray!(w_ray, |row, col, offset| (row, col-offset));
define_ray!(s_ray, |row, col, offset| (row-offset, col));
define_ray!(sw_ray, |row, col, offset| (row-offset, col-offset));
define_ray!(se_ray, |row, col, offset| (row-offset, col+offset));

impl Rays {
    fn initialize() -> Self {
        let n_rays = make_rays!(n_ray);
        let e_rays = make_rays!(e_ray);
        let nw_rays = make_rays!(nw_ray);
        let ne_rays = make_rays!(ne_ray);
        let w_rays = make_rays!(w_ray);
        let s_rays = make_rays!(s_ray);
        let sw_rays = make_rays!(sw_ray);
        let se_rays = make_rays!(se_ray);
        Self {
            n_rays,
            e_rays,
            nw_rays,
            ne_rays,
            w_rays,
            s_rays,
            sw_rays,
            se_rays,
        }
    }
}

fn print_bitboard_to_string(bitboard: BitBoard, mark: Option<usize>) -> String {
    let mut row = "".to_owned();
    let mut board = "".to_owned();

    for i in 0..64 {
        let value = (bitboard>>i)&1;
        let s = if value == 0 {
            ".".to_owned()
        } else {
            value.to_string()
        };
        match mark {
            Some(idx) => if i == idx {
                row.push_str("X");
            } else {
                row.push_str(&s);
            },
            None => row.push_str(&s),
        }
        if (i+1) % 8 == 0 {
            row.push_str("\n");
            board.insert_str(0, &row);
            row.clear();
        }
    }
    board
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_n_ray(){
        let rays = Rays::initialize();
        let row = 5;
        let col = 4;
        let idx = (row-1)*8+col-1;
        println!("{}",print_bitboard_to_string(rays.n_rays[idx], Some(idx)))
    }

    #[test]
    fn make_e_ray(){
        let rays = Rays::initialize();
        let row = 5;
        let col = 4;
        let idx = (row-1)*8+col-1;
        println!("{}",print_bitboard_to_string(rays.e_rays[idx], Some(5*8+4)))
    }

    #[test]
    fn make_ne_ray(){
        let rays = Rays::initialize();
        let row = 5;
        let col = 4;
        let idx = (row-1)*8+col-1;
        println!("{}",print_bitboard_to_string(rays.ne_rays[idx], Some(5*8+4)))
    }

    #[test]
    fn make_nw_ray(){
        let rays = Rays::initialize();
        let row = 5;
        let col = 4;
        let idx = (row-1)*8+col-1;
        println!("{}",print_bitboard_to_string(rays.nw_rays[idx], Some(idx)))
    }

    #[test]
    fn make_s_ray(){
        let rays = Rays::initialize();
        let row = 5;
        let col = 4;
        let idx = (row-1)*8+col-1;
        println!("{}",print_bitboard_to_string(rays.s_rays[idx], Some(idx)))
    }

    #[test]
    fn make_w_ray(){
        let rays = Rays::initialize();
        let row = 5;
        let col = 4;
        let idx = (row-1)*8+col-1;
        println!("{}",print_bitboard_to_string(rays.w_rays[idx], Some(idx)))
    }

    #[test]
    fn make_sw_ray(){
        let rays = Rays::initialize();
        let row = 5;
        let col = 4;
        let idx = (row-1)*8+col-1;
        println!("{}",print_bitboard_to_string(rays.sw_rays[idx], Some(idx)))
    }

    #[test]
    fn make_se_ray(){
        let rays = Rays::initialize();
        let row = 5;
        let col = 4;
        let idx = (row-1)*8+col-1;
        println!("{}",print_bitboard_to_string(rays.se_rays[idx], Some(idx)))
    }

}