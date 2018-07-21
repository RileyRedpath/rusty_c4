use c4::*;

#[derive(Clone)]
pub struct Leaf {
    pub winner: Player,
}

pub enum Node {
    InnerNode(InnerNode),
    Leaf(Leaf),
}

pub struct InnerNode {
    pub board: Board,
    pub children: Vec<Branch>,
    pub turn: Player,
}

pub struct Branch {
    pub input: u32,
    pub node: Node,
}

impl InnerNode {
    pub fn find_children(&mut self) {
        for i in 0..self.board.w {
            let mut new_board = self.board.place(i, self.turn);
            if let Some(mut b) = new_board {
                let branch;
                if b.is_over(i) {
                    branch = Branch {
                        node: Node::Leaf(Leaf { winner: self.turn }),
                        input: i,
                    }
                } else if b.turn_number >= b.w * b.h {
                    branch = Branch {
                        node: Node::Leaf(Leaf {
                            winner: Player::Empty,
                        }),
                        input: i,
                    }
                } else {
                    branch = Branch {
                        node: Node::InnerNode(InnerNode::new(b, self.turn.switch())),
                        input: i,
                    }
                }
                self.children.push(branch)
            }
        }
    }

    pub fn new(board: Board, turn: Player) -> InnerNode {
        InnerNode {
            board: board,
            children: Vec::new(),
            turn: turn,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn leaf_on_win() {
        let b = Board::from_int_array(vec![1, 1, 1, 0, 0], 5, 1);
        let mut root = InnerNode::new(b, Player::P1);

        root.find_children();
        assert_eq!(root.children.len(), 2);

        let branch1 = &root.children[0];
        let branch2 = &root.children[1];

        assert_eq!(branch1.input, 3);
        assert_eq!(branch2.input, 4);

        match branch1.node {
            Node::Leaf(ref l) => assert_eq!(l.winner, Player::P1),
            Node::InnerNode(ref _i) => panic!(),
        }

        match branch2.node {
            Node::Leaf(ref _l) => panic!(),
            Node::InnerNode(ref i) => {
                assert_eq!(i.turn, Player::P2);
                assert_eq!(i.board.turn_number, root.board.turn_number + 1);
            }
        }
    }

    #[test]
    fn leaf_on_full() {
        let b = Board::from_int_array(vec![1, 1, 1, -1, 0], 5, 1);
        let mut root = InnerNode::new(b, Player::P1);

        root.find_children();

        assert_eq!(root.children.len(), 1);

        let branch = &root.children[0];
        assert_eq!(branch.input, 4);

        match branch.node {
            Node::Leaf(ref l) => assert_eq!(l.winner, Player::Empty),
            Node::InnerNode(ref _i) => panic!(),
        }
    }

    #[test]
    fn win_on_last_placement() {
        let b = Board::from_int_array(vec![1, 1, 1, 0], 4, 1);

        let mut root = InnerNode::new(b, Player::P1);

        root.find_children();

        assert_eq!(root.children.len(), 1);

        let branch = &root.children[0];
        assert_eq!(branch.input, 3);

        match branch.node {
            Node::Leaf(ref l) => assert_eq!(l.winner, Player::P1),
            Node::InnerNode(ref _i) => panic!(),
        }

        let bb = Board::from_int_array(vec![-1, -1, 0, 1, 1, 1, 0],7,1);
        let mut root2 = InnerNode::new(bb, Player::P2);
        root2.find_children();
        assert_eq!(root2.children.len(), 2);
        assert_eq!(root2.children[1].input, 6);
        match &mut(root2.children[1]).node {
            Node::Leaf(ref _l) => panic!(),
            Node::InnerNode(ref mut i) => {
                i.find_children();
                i.board.print_board();
                assert_eq!(i.children.len(), 1);
                assert_eq!(i.children[0].input, 2);
                match &(i.children[0]).node {
                    Node::Leaf(ref l) => assert_eq!(l.winner, Player::P1),
                    Node::InnerNode(ref _i) => panic!(),
                }
            },
        }
    }
}
