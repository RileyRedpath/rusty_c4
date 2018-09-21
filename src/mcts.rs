use c4::*;
use tree::*;
use rollout::*;


pub fn mcts(board: &Board, p: Player) -> u32 {
    let mut n = InnerNode::new(board.clone(), p);
    n.find_children();

    let mut step_data = StepData::new(p);

    for child in n.children {
        let score = mcts_step(child.node, step_data.next());
        if step_data.update(score, child.input){ 
            break;
        }
    }
    step_data.best_move.expect("no move found")
}

fn mcts_step(n: Node, mut step_data: StepData) -> f32 {
    match n {
        Node::Leaf(leaf) => {
            return if leaf.winner == Player::P1 {
                1.
            } else if leaf.winner == Player::P2 {
                -1.
            } else {
                0.
            };
        }
        Node::InnerNode(mut node) => {
            if step_data.d == 0 {
                //return rollout(&node.board, &node.turn);
                return average_random_rollout(&node.board, &node.turn, 10);
            }

            node.find_children();
            for child in node.children {
                let score = 0.9 * mcts_step(child.node, step_data.next());
                if step_data.update(score, child.input){ 
                    break;
                }
            }
            return step_data.v;
        }
    }
}

fn maximizing_fn(v: f32, score: f32, alpha: f32, beta: f32) -> (f32, f32, f32) {
    return (score.max(v), alpha.max(v), beta);
}

fn minimizing_fn(v: f32, score: f32, alpha: f32, beta: f32) -> (f32, f32, f32) {
    return (score.min(v), alpha, beta.min(v));
}

struct StepData {
    v: f32,
    a: f32,
    b: f32,
    d: i32,
    compare_fn: fn(f32, f32, f32, f32) -> (f32, f32, f32),
    p: Player,
    best_move: Option<u32>,
}

impl StepData {
    pub fn new(player: Player) -> StepData {
        match player {
            Player::P1 => {
                return StepData {
                    v: -2.0,
                    a: -2.0,
                    b: 2.0,
                    d: 5,
                    compare_fn: maximizing_fn,
                    p: Player::P1,
                    best_move: None,
                }
            }
            _ => {
                return StepData {
                    v: 2.0,
                    a: -2.0,
                    b: 2.0,
                    d: 5,
                    compare_fn: minimizing_fn,
                    p: Player::P2,
                    best_move: None,
                }
            }
        }
    }

    fn update(&mut self, score: f32, current_move: u32) -> bool {
        if let None = self.best_move { self.best_move = Some(current_move)};
        let (v_new, a_new, b_new) = (self.compare_fn)(self.v, score, self.a, self.b);
        if self.v < v_new && self.p == Player::P1 || self.v > v_new && self.p == Player::P2 {
            self.v = v_new;
            self.best_move = Some(current_move);
        }
        self.a = a_new;
        self.b = b_new;
        if self.b <= self.a {
            self.best_move = Some(current_move);
            return true;
        }
        return false;
    }

    fn next(&self) -> StepData {
        match self.p {
            Player::P1 => return StepData {
                v: 2.0,
                a: self.a,
                b: self.b,
                d: self.d - 1,
                compare_fn: minimizing_fn,
                p: Player::P2,
                best_move: None,
            },
            _ => return StepData {
                v: -2.0,
                a: self.a,
                b: self.b,
                d: self.d - 1,
                compare_fn: maximizing_fn,
                p: Player::P1,
                best_move: None,
            },
        };
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn mcts_test() {
        let vector = vec![-1, -1, 0, 0, 1, 1, 0];
        let board = Board::from_int_array(vector,7,1);
        let k = mcts(&board, Player::P1);
        assert_eq!(k, 3);
    }

    #[test]
    fn big_mcts_test() {
        let vector = vec![0; 49];
        let board = Board::from_int_array(vector,7,7);
        let k = mcts(&board, Player::P1);
        assert!(k != 0);
    }

    //#[test]
    fn big_game() {
        let mut b = Board::from_int_array(vec![0; 49], 7, 7);
        let mut p = Player::P1;
        for i in 0..49 {
            let mut vec = vec![0; 49];
            for j in 0..7 {
                for k in 0..7 {
                    match b.get(k, j) {
                        Player::P1 => vec[(j * 7 + k) as usize] = 1,
                        Player::P2 => vec[(j * 7 + k) as usize] = -1,
                        Player::Empty => (),
                    }
                }
            }
            let k = mcts(&Board::from_int_array(vec,7,7), Player::P1);
            let b_opt = b.place(k, p);
            match b_opt {
                Some(bb) => {
                    b = bb;
                    b.print_board();
                    print!("\n\n ==================\n\n");
                    if b.is_over(k) {
                        break;
                    }
                    if b.turn_number >= 49 {
                        p = Player::Empty;
                        break;
                    }
                    p = p.switch();
                }
                None => panic!(""),
            }
        }
        print!("\n\n ***WINNER***{:?}***\n\n", p);
    }
    
}

