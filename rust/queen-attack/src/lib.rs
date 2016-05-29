pub struct ChessPosition(i8, i8);

pub struct Queen {
    position: ChessPosition
}

impl ChessPosition {
    pub fn new(x: i8, y: i8) -> Result<ChessPosition, &'static str> {
        if ChessPosition::is_on_board(x, y) {
            Ok(ChessPosition(x, y))
        } else {
            Err("Invalid Position")
        }
    }

    fn is_on_board(x: i8, y: i8) -> bool {
        x >= 0 && x < 8 && y >= 0 && y < 8
    }
}

impl Queen {
    pub fn new(pos: ChessPosition) -> Queen {
        Queen { position: pos }
    }

    pub fn can_attack(&self, queen: &Queen) -> bool {
        self.is_same_rank(queen) ||
            self.is_same_file(queen) ||
            self.is_same_diagonal(queen)
    }

    fn is_same_rank(&self, queen: &Queen) -> bool {
        queen.position.0 == self.position.0
    }

    fn is_same_file(&self, queen: &Queen) -> bool {
        queen.position.1 == self.position.1
    }

    fn is_same_diagonal(&self, queen: &Queen) -> bool {
        (queen.position.1 - self.position.1).abs() ==
            (queen.position.0 - self.position.0).abs()
    }
}
