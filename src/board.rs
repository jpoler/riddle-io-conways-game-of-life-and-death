use types::Square;
use Square::*;

pub struct Board {
    state: Vec<Vec<Square>>,
    rows: usize,
    cols: usize,
}

impl Board {
    pub fn from_state(state: &Vec<Square>, rows: usize, cols: usize) -> Board {
        if rows * cols != state.len() {
            panic!(
                "incompatible parameters: state length '{}', rows '{}' cols '{}'",
                state.len(),
                rows,
                cols
            );
        }
        let mut inner = Vec::with_capacity(cols);
        for i in 0..rows {
            let idx = i * cols;
            let mut row = Vec::with_capacity(rows);
            for j in 0..cols {
                row.push(state[idx + j]);
            }
            inner.push(row);
        }

        Board {
            state: inner,
            rows,
            cols,
        }
    }

    pub fn state(&self) -> Vec<Square> {
        self.state.clone().into_iter().flatten().collect()
    }

    fn rc_coordinate(&self, r: usize, c: usize) -> Option<Square> {
        if r >= self.rows || c >= self.cols {
            None
        } else {
            Some(self.state[r][c])
        }
    }

    fn xy_coordinate(&self, x: usize, y: usize) -> Option<Square> {
        self.rc_coordinate(y, x)
    }

    fn neighbor_rc_coordinates(&self, r: i64, c: i64) -> Vec<(i64, i64)> {
        vec![
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ].into_iter()
        .filter(|(i, j)| {
            // YAARG rustfmt
            r + i >= 0 && r + i < (self.rows as i64) && c + j >= 0 && c + j < (self.cols as i64)
        }).collect::<Vec<(i64, i64)>>()
    }

    pub fn simulate_step(&mut self) {
        let mut new_state = self.state.clone();

        // this would be super fun to do in simd (though x86 simd is trash to work with)

        for r in 0..self.rows {
            for c in 0..self.cols {
                let mut neighbors = 0;
                let mut p1 = 0;
                let mut p2 = 0;
                for (i, j) in self.neighbor_rc_coordinates(r as i64, c as i64) {
                    match self.state[(r as i64 + i) as usize][(c as i64 + j) as usize] {
                        Square::Empty => {}
                        Square::Player1 => {
                            neighbors += 1;
                            p1 += 1;
                        }
                        Square::Player2 => {
                            neighbors += 1;
                            p2 += 1;
                        }
                    }
                }

                new_state[r][c] = match (self.state[r][c], neighbors) {
                    (Player1, 0...1) | (Player2, 0...1) => Square::Empty,
                    (p @ Player1, 2...3) | (p @ Player2, 2...3) => p,
                    (Player1, _) | (Player2, _) => Empty,
                    (Empty, 3) => if p1 > p2 {
                        Player1
                    } else {
                        Player2
                    },
                    (Empty, _) => Empty,
                }
            }
        }

        self.state = new_state;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_state() {
        let input = vec![
            Square::Empty,
            Square::Player1,
            Square::Player2,
            Square::Empty,
        ];
        let board = Board::from_state(&input, 2, 2);
        assert_eq!(Square::Empty, board.xy_coordinate(0, 0).unwrap());
        assert_eq!(Square::Player1, board.xy_coordinate(1, 0).unwrap());
        assert_eq!(Square::Player2, board.xy_coordinate(0, 1).unwrap());
        assert_eq!(Square::Empty, board.xy_coordinate(1, 1).unwrap());
    }

    #[test]
    fn rc_coordinate() {
        let input = vec![
            Square::Empty,
            Square::Player1,
            Square::Player2,
            Square::Empty,
        ];
        let board = Board::from_state(&input, 2, 2);

        assert_eq!(None, board.rc_coordinate(2, 0));
        assert_eq!(None, board.rc_coordinate(0, 2));
    }

    #[test]
    fn neighbor_rc_coordinates() {
        let input = vec![
            Square::Empty,
            Square::Empty,
            Square::Empty,
            Square::Empty,
            Square::Empty,
            Square::Empty,
            Square::Empty,
            Square::Empty,
            Square::Empty,
        ];
        let board = Board::from_state(&input, 3, 3);

        assert_eq!(board.neighbor_rc_coordinates(0, 0).len(), 3);
        assert_eq!(board.neighbor_rc_coordinates(0, 1).len(), 5);
        assert_eq!(board.neighbor_rc_coordinates(0, 2).len(), 3);
        assert_eq!(board.neighbor_rc_coordinates(1, 0).len(), 5);
        assert_eq!(board.neighbor_rc_coordinates(1, 1).len(), 8);
        assert_eq!(board.neighbor_rc_coordinates(1, 2).len(), 5);
        assert_eq!(board.neighbor_rc_coordinates(2, 0).len(), 3);
        assert_eq!(board.neighbor_rc_coordinates(2, 1).len(), 5);
        assert_eq!(board.neighbor_rc_coordinates(2, 2).len(), 3);
    }

    #[test]
    fn simulate_step() {
        // for now just make sure it doesn't panic
        // I'll be testing this against the actual game engine output
        let input = vec![
            Square::Empty,
            Square::Player1,
            Square::Player2,
            Square::Empty,
        ];
        let mut board = Board::from_state(&input, 2, 2);

        board.simulate_step();

        let expected = vec![Square::Empty, Square::Empty, Square::Empty, Square::Empty];
        let actual = board.state.iter().flatten();

        assert_eq!(expected.iter().eq(actual), true);

        let input = vec![
            Square::Empty,
            Square::Empty,
            Square::Empty,
            Square::Empty,
            Square::Empty,
            Square::Player1,
            Square::Player1,
            Square::Empty,
            Square::Empty,
            Square::Player1,
            Square::Player1,
            Square::Empty,
            Square::Empty,
            Square::Empty,
            Square::Empty,
            Square::Empty,
        ];
        let mut board = Board::from_state(&input, 4, 4);

        board.simulate_step();

        let expected = vec![
            Square::Empty,
            Square::Empty,
            Square::Empty,
            Square::Empty,
            Square::Empty,
            Square::Player1,
            Square::Player1,
            Square::Empty,
            Square::Empty,
            Square::Player1,
            Square::Player1,
            Square::Empty,
            Square::Empty,
            Square::Empty,
            Square::Empty,
            Square::Empty,
        ];

        let actual = board.state.iter().flatten();
        assert_eq!(expected.iter().eq(actual), true);
        println!("really?")
    }
}
