extern crate rand;

use c4::*;

use rand::{Rng, SeedableRng, XorShiftRng};
use rayon::prelude::*;


pub fn average_random_rollout(board_orig: &Board, p_orig: &Player, n: u32) -> f32 {
    //let mut rng: XorShiftRng = SeedableRng::from_seed([1,2,3,4]);
    //let mut f = move |x| rng.gen_range(0, x);
    let f = move |y| {
        let mut rng: XorShiftRng =
            SeedableRng::from_seed([y + 1, y ^ 2, y * 3, (y + 2) * (8 + y ^ 2)]);
        move |x| rng.gen_range(0, x)
    };
    let cumulative: f32;

    /*
    for _ in 0..n {
        cumulative += random_rollout(board_orig, p_orig, &mut f);
    }
    */

    cumulative = (0..n)
        .into_par_iter()
        .map(|x| random_rollout(board_orig, p_orig, &mut f(x)))
        .sum();
    cumulative / (n as f32)
}

fn random_rollout(board_orig: &Board, p_orig: &Player, rng: &mut FnMut(u32) -> u32) -> f32 {
    let mut board = board_orig.clone();
    let mut p = p_orig.clone();

    let mut possible_moves = Vec::new();
    for i in 0..board.w {
        possible_moves.push(i);
    }

    let size = board.w * board.h;

    for _ in 0..size + 1 {
        let i: u32 = rng(possible_moves.len() as u32);
        let board_option = board.place(i, p);

        match board_option {
            None => {
                possible_moves.remove_item(&i);
            }
            Some(b) => {
                board = b;

                if board.is_over(i) {
                    break;
                }
                p = p.switch();
            }
        }

        if board.turn_number >= size || possible_moves.len() == 0 {
            p = Player::Empty;
            break;
        }
    }

    match p {
        Player::P1 => 1.,
        Player::P2 => -1.,
        Player::Empty => 0.,
    }
}
