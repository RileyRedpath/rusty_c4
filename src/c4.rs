#[derive(Clone)]
pub struct Board {
    pub w: u32,
    pub h: u32,
    pub turn_number: u32,
    pub vector: Vec<Player>,
    over: Option<bool>,
}

pub fn assert_log(message: &str, condition: bool){
    if !condition {
        print!("{}\n",message);
        panic!(format!("{}",message));
    }
}

impl Board {
    pub fn get(&self, column: u32, row: u32) -> Player {
        assert_log("column greater than width, in get", column < self.w);
        assert_log("row greater than height, in get", row < self.h);
        self.vector[(row * self.w + column) as usize]
    }

    fn get_option(&self, column: i32, row: i32) -> Option<Player> {
        if (column as u32) >= self.w {
            return None;
        };
        if (row as u32) >= self.h {
            return None;
        };
        if column < 0 {
            return None;
        };
        if row < 0 {
            return None;
        };
        Some(self.vector[((row as u32) * self.w + (column as u32)) as usize])
    }

    fn put(&mut self, column: u32, row: u32, p: Player) {
        assert_log("column greater than width, in put", column < self.w);
        assert_log("row greater than height, in put", row < self.h);

        self.vector[(row * self.w + column) as usize] = p;
    }

    pub fn place(&self, column: u32, p: Player) -> Option<Board> {
        let column_floor = self.find_column_floor(column);

        if column_floor == self.h {
            //column full
            return None;
        }

        let mut new_board = (*self).clone();

        new_board.turn_number = self.turn_number + 1;
        new_board.over = None;

        new_board.put(column, column_floor, p);

        return Some(new_board);
    }

    pub fn is_over(&mut self, x: u32) -> bool {
        if let Some(b) = self.over {
            return b;
        }

        let y = self.find_column_floor(x) - 1;
        let p = self.get(x, y);

        //vertical win
        let (y_lower_start_bound, y_upper_start_bound) = find_bounds_for_line(y as i32, self.h, 4);
        for i in y_lower_start_bound..y_upper_start_bound {
            //if all 4 pieces are p, there is a winning connection
            if self.get(x, i) == p && self.get(x, i + 1) == p && self.get(x, i + 2) == p
                && self.get(x, i + 3) == p
            {
                self.over = Some(true);
                return true;
            }
        }

        //horizontal win
        let (x_lower_start_bound,x_upper_start_bound) = find_bounds_for_line(x as i32, self.w, 4);
        for i in x_lower_start_bound..x_upper_start_bound{
            //if all 4 pieces are p, there is a winning connection
            if self.get(i, y) == p && self.get(i + 1, y) == p && self.get(i + 2, y) == p
                && self.get(i + 3, y) == p
            {
                self.over = Some(true);
                return true;
            }
        }

        //diagonal win
        //two diagonal lines, dx/dy = 1 and dx/dy = -1
        for j in vec![-1, 1] {
            //four possible winning connections including the latest move
            for i in 0..4 as i32 {
                let mut diagonal_win = true;
                //four places in each winning connection
                for k in 0..4 as i32 {
                    let diag_x = x as i32 + i - k;
                    let diag_y = y as i32 + j * (i - k);
                    let cell = self.get_option(diag_x, diag_y);

                    match cell {
                        //cell is None if out of bounds, current winning connection is invalid
                        None => {
                            diagonal_win = false;
                            break;
                        }
                        Some(piece_type) => {
                            //no winning connection here
                            if piece_type != p {
                                diagonal_win = false;
                                break;
                            }
                        }
                    }
                }
                if diagonal_win {
                    self.over = Some(true);
                    return true;
                }
            }
        }

        self.over = Some(false);
        return false;
    }

    fn find_column_floor(&self, column: u32) -> u32 {
        for i in (0..self.h).rev() {
            match self.get(column, i) {
                Player::Empty => (),
                _ => return i + 1,
            };
        }

        return 0;
    }

    pub fn new(array: Vec<Player>, turn_number: u32, w: u32, h: u32) -> Board {
        Board {
            vector: array,
            turn_number: turn_number,
            over: None,
            w: w,
            h: h,
        }
    }

    pub fn from_int_array(board_array: Vec<i32>, w: u32, h: u32) -> Board {
        let mut board_array_player = Vec::new();

        let mut turn_number = 0;
        for place in board_array {
            let player_in_place = if place > 0 {
                turn_number += 1;
                Player::P1
            } else if place < 0 {
                turn_number += 1;
                Player::P2
            } else {
                Player::Empty
            };
            board_array_player.push(player_in_place);
        }

        Board::new(board_array_player, turn_number, w, h)
    }

    pub fn print_board(&self) {
        for i in (0..self.h).rev() {
            for k in 0..self.w {
                match self.get(k, i) {
                    Player::P1 => print!("X "),
                    Player::P2 => print!("Y "),
                    Player::Empty => print!("0 "),
                }
            }
            print!("\n");
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Player {
    P1 = -1,
    Empty = 0,
    P2 = 1,
}

impl Player {
    pub fn switch(&self) -> Player {
        match *self {
            Player::P1 => Player::P2,
            Player::P2 => Player::P1,
            Player::Empty => Player::Empty,
        }
    }
}

fn clamp(a: i32, b: u32) -> u32 {
    if a < 0 {
        0
    } else if a >= (b as i32) {
        b
    } else {
        a as u32
    }
}

fn find_bounds_for_line(a: i32, bound: u32, length: u32) -> (u32,u32){
    let diff = (length - 1) as i32;
    let earliest = clamp(a - diff, bound - 1);
    let latest = clamp(clamp(a + diff, bound - 1) as i32 - diff + 1, bound - 1);
    (earliest, latest)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn place() {
        let b = Board::from_int_array(vec![0; 49], 7, 7);

        let new_b_option = b.place(3, Player::P1);

        match new_b_option {
            Some(new_b) => assert!(new_b.get(3, 0) == Player::P1),
            None => panic!("board should have returned as Some"),
        }
    }

    #[test]
    fn find_column_floor() {
        let mut b = Board::from_int_array(vec![0; 49], 7, 7);
        b.put(0, 0, Player::P1);

        assert_eq!(b.find_column_floor(0), 1);
    }

    #[test]
    fn board_put() {
        let mut b = Board::from_int_array(vec![0; 49], 7, 7);
        b.put(0, 0, Player::P1);

        assert!(b.get(0, 0) == Player::P1);
    }

    #[test]
    fn board_from_int_array() {
        let b = Board::from_int_array(vec![0; 49], 7, 7);

        assert_eq!(b.h, 7);
        assert_eq!(b.w, 7);
        assert_eq!(b.vector.len(), 49);
    }

    #[test]
    fn win() {
        let mut b = Board::from_int_array(vec![0; 49], 7, 7);

        b.put(0, 0, Player::P1);
        b.put(1, 0, Player::P1);
        b.put(2, 0, Player::P1);
        b.put(3, 0, Player::P1);

        assert!(b.is_over(3));

        b = Board::from_int_array(vec![0; 49], 7, 7);

        b.put(0, 0, Player::P1);
        b.put(0, 1, Player::P1);
        b.put(0, 2, Player::P1);
        b.put(0, 3, Player::P1);

        assert!(b.is_over(0));

        b = Board::from_int_array(vec![0; 49], 7, 7);

        b.put(0, 0, Player::P1);
        b.put(1, 1, Player::P1);
        b.put(2, 2, Player::P1);
        b.put(3, 3, Player::P1);

        assert!(b.is_over(3));

        b = Board::from_int_array(vec![1,1,1,1], 4, 1);
        assert!(b.is_over(2));

    }

    #[test]
    fn bounds_test(){
        bounds_case(1, 4, 4, 7, 4);
        bounds_case(0, 1, 2, 4, 4);
        bounds_case(0, 0, 2, 7, 20);
        bounds_case(0, 3, 2, 7, 4);
    }

    fn bounds_case(expected_lower: u32, expected_higher: u32, x: i32, bound: u32, length: u32){
        let (a,b) = find_bounds_for_line(x, bound, length);
        assert_eq!(a, expected_lower);
        assert_eq!(b, expected_higher);
    }

}
