const NCOLS: u8 = 7;
const NROWS: u8 = 6;
const NLINE: u8 = 4;

pub mod minmax;
pub mod patterns;

use self::patterns::PatternsCountPlayerPonderation as PCPP;
use self::patterns::PatternsCountPonderation as PCP;

#[cfg(test)]
mod test;

fn pattern_ponderation() -> PCPP {
    PCPP {
        player_current: PCP {
            next_move_wins: 1.0,
            imposible_avoid: 10.0,
            vert_consecutive_hole_3inline: 5.0,
            line3: 1.0,
            line2: 0.1,
            line1: 0.01,
        },
        player_other: PCP {
            next_move_wins: 100.0,
            imposible_avoid: 10.0,
            vert_consecutive_hole_3inline: 5.0,
            line3: 1.0,
            line2: 0.1,
            line1: 0.01,
        },
    }
    // PCPP {
    //     player_current: PCP {
    //         next_move_wins: 1.0,
    //         imposible_avoid: 55.5,
    //         vert_consecutive_hole_3inline: 0.3,
    //         line3: 0.1,
    //         line2: 0.01,
    //         line1: 0.001,
    //     },
    //     player_other: PCP {
    //         next_move_wins: 100.0,
    //         imposible_avoid: 55.5,
    //         vert_consecutive_hole_3inline: 0.3,
    //         line3: 0.1,
    //         line2: 0.01,
    //         line1: 0.001,
    //     },
    // }
}

/// Abstract type with game status
///
pub struct Game {
    pub board: Board,
    pub turn: Turn,
    pub patterns: patterns::Patterns,
    pub patterns_pond: patterns::PatternsCountPlayerPonderation,
    pub moves: Vec<Col>,
}

/// Player to move, or winner of game
///
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Turn {
    P(Player),
    F(Finished),
    // Won(Player),
    // Draw(Player),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Finished {
    Won(Player),
    Draw(Player),
}

/// Player options
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Player {
    O,
    X,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Col(u8);
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Row(u8);

/// Abstract type to manage the board
#[derive(PartialEq, Eq)]
pub struct Board([[Cell; NCOLS as usize]; NROWS as usize]);

/// Two players, O and X
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Cell {
    P(Player),
    Empty,
}

impl Col {
    pub fn b(v: u8) -> Option<Self> {
        if v < NCOLS {
            Some(Col(v))
        } else {
            None
        }
    }
}

impl std::fmt::Display for Col {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "c{}", self.0)
    }
}

impl Row {
    pub fn b(v: u8) -> Option<Self> {
        if v < NROWS {
            Some(Row(v))
        } else {
            None
        }
    }
}

impl Game {
    /// ```rust
    /// extern crate connect4;
    ///
    /// use connect4::engine::*;
    ///
    /// fn main() {
    ///     let game = Game::new(Player::X);
    ///
    ///     let board = connect4::engine::board_from_string(
    ///         "
    ///          _______
    ///          _______
    ///          _______
    ///          _______
    ///          _______
    ///          _______
    ///         ",
    ///     );
    ///
    ///     println!("{}", game);
    ///
    ///     assert!(game.board == board.unwrap());
    ///     if let Turn::P(next_player) = game.turn {
    ///         assert!(next_player == Player::X)
    ///     } else {
    ///         panic!("error on next move")
    ///     }
    /// }
    /// ```
    pub fn new(start: Player) -> Game {
        Game {
            board: empty_board(),
            turn: Turn::P(start),
            patterns: patterns::Patterns::init(),
            patterns_pond: pattern_ponderation(),
            moves: vec![],
        }
    }

    pub fn set_patterns_pond(mut self, pcpp: PCPP) -> Self {
        self.patterns_pond = pcpp;
        self
    }

    /// Example with one movement
    ///
    /// ```rust
    /// extern crate connect4;
    ///
    /// use connect4::engine::*;
    ///
    /// fn main() {
    ///     let game = Game::new(Player::O);
    ///
    ///     let board = connect4::engine::board_from_string(
    ///         "
    ///          _______
    ///          _______
    ///          _______
    ///          _______
    ///          _______
    ///          O______
    ///         ",
    ///     );
    ///
    ///     let play_col = |game: Game, col| -> Game {
    ///         match game.play(Col::b(col).unwrap()) {
    ///             Ok(game) => game,
    ///             _ => panic!("error processing move"),
    ///         }
    ///     };
    ///
    ///     let game = play_col(game, 0);
    ///
    ///     println!("{}", game);
    ///
    ///     assert!(game.board == board.unwrap());
    ///     if let Turn::P(next_player) = game.turn {
    ///         assert!(next_player == Player::X)
    ///     } else {
    ///         panic!("error on next move")
    ///     }
    /// }
    /// ```
    ///
    /// Example with several moves
    ///
    /// ```rust
    /// extern crate connect4;
    ///
    /// use connect4::engine::*;
    ///
    /// fn main() {
    ///     let game = Game::new(Player::X);
    ///
    ///     let board = connect4::engine::board_from_string(
    ///         "
    ///          _______
    ///          _______
    ///          _____O_
    ///          _____X_
    ///          _____O_
    ///          _____X_
    ///         ",
    ///     );
    ///
    ///     let play_col = |game: Game, col| -> Game {
    ///         match game.play(Col::b(col).unwrap()) {
    ///             Ok(game) => game,
    ///             _ => panic!("error processing move"),
    ///         }
    ///     };
    ///
    ///     let game = play_col(game, 5);
    ///     let game = play_col(game, 5);
    ///     let game = play_col(game, 5);
    ///     let game = play_col(game, 5);
    ///
    ///     println!("{}", game);
    ///
    ///     assert!(game.board == board.unwrap());
    ///     if let Turn::P(next_player) = game.turn {
    ///         assert!(next_player == Player::X)
    ///     } else {
    ///         panic!("error on next move")
    ///     }
    /// }
    /// ```
    ///
    /// What if you try to play when column is exausted???
    /// You get an error
    ///
    /// ```rust
    /// extern crate connect4;
    ///
    /// use connect4::engine::*;
    ///
    /// fn main() {
    ///     let game = Game::new(Player::X);
    ///
    ///     let play_col = |game: Game, col| -> Game {
    ///         match game.play(Col::b(col).unwrap()) {
    ///             Ok(game) => game,
    ///             _ => panic!("error processing move"),
    ///         }
    ///     };
    ///
    ///     let game = play_col(game, 5);
    ///     let game = play_col(game, 5);
    ///     let game = play_col(game, 5);
    ///     let game = play_col(game, 5);
    ///     let game = play_col(game, 5);
    ///     let game = play_col(game, 5);
    ///
    ///     let egame = game.play(Col::b(5).unwrap());
    ///
    ///     assert!(egame.is_err())
    /// }
    /// ```
    pub fn play(mut self, col: Col) -> std::result::Result<Game, Game> {
        let next_turn = |patterns: &patterns::Patterns, player, nmoves| {
            let switch_player = |player| match player {
                Player::O => Player::X,
                Player::X => Player::O,
            };
            let full = nmoves == (NROWS as usize) * (NCOLS as usize);
            match (patterns, full) {
                (patterns::Patterns::FourInLine, _) => Turn::F(Finished::Won(player)),
                (_, true) => Turn::F(Finished::Draw(player)),
                (patterns::Patterns::P(ref _pc), _) => Turn::P(switch_player(player)),
            }
        };

        //  -------
        match (self.board.row_for_play(col), next_player(&self)) {
            (Some(row), Some(player)) => {
                self.board.0[row.0 as usize][col.0 as usize] = Cell::P(player);
                self.patterns = patterns::get_patterns(&self.board);
                self.moves.push(col);
                self.turn = next_turn(&self.patterns, player, self.moves.len());
                Ok(self)
            }
            _ => Err(self),
        }
    }

    pub fn try_play(self, col: Col) -> Self {
        match self.play(col) {
            Ok(game) => game,
            Err(game) => game,
        }
    }

    pub fn eval(&self) -> patterns::Eval {
        self.patterns.eval(self.turn, &self.patterns_pond)
    }

    pub fn undo(mut self) -> std::result::Result<Game, Game> {
        let switch_player = |t: Turn| match t {
            Turn::P(Player::O) => Turn::P(Player::X),
            Turn::P(Player::X) => Turn::P(Player::O),
            Turn::F(Finished::Won(Player::O)) => Turn::P(Player::O),
            Turn::F(Finished::Won(Player::X)) => Turn::P(Player::X),
            Turn::F(Finished::Draw(Player::O)) => Turn::P(Player::O),
            Turn::F(Finished::Draw(Player::X)) => Turn::P(Player::X),
        };
        //  -------
        match self.moves.pop() {
            Some(col) => {
                if self.board.remove_from_col(col) {
                    self.turn = switch_player(self.turn);
                    Ok(self)
                } else {
                    Err(self)
                }
            }
            None => Err(self),
        }
    }
}

fn next_player(game: &Game) -> Option<Player> {
    if let Turn::P(ref player) = game.turn {
        Some(*player)
    } else {
        None
    }
}

/// 6 lines, spaces at begin or end of line
/// \n to separate lines
///
/// In next example, we test the symmetry
///     board -> string -> board
///     
/// To simplify we start with a string
///
/// >    string1 -> board1 -> string2 -> board2
///
/// board1 == board2
///
///
/// ```rust
/// extern crate connect4;
///
///
/// fn main() {
///     let board = connect4::engine::board_from_string(
///         "
///         _______
///         _______
///         __O____
///         __O____
///         __OX___
///         __OOXX_
/// ",
///     ).unwrap();
///
///     let bstring = format!("{}", board);
///     let new_board = connect4::engine::board_from_string(&bstring).unwrap();
///
///     assert!(board == new_board);
///
///     println!("{}", board);
/// }
/// ```
pub fn board_from_string(blines: &str) -> Option<Board> {
    let (_, board) = blines
        .lines()
        .try_fold((0, empty_board()), |(row, board), line| {
            let line = line.trim();
            if !line.is_empty() {
                let board = sline2board_row(board, line, row)?;
                Some((row + 1, board))
            } else {
                Some((row, board))
            }
        })?;

    Some(board)
}

impl Board {
    pub fn get_cell(&self, col: Col, row: Row) -> Cell {
        self.get_cell_dangerous(col.0 as usize, row.0 as usize)
    }

    pub(crate) fn get_cell_dangerous(&self, col: usize, row: usize) -> Cell {
        self.0[row][col]
    }

    /// Returns the valid column to play starting from col
    ///
    /// ```rust
    /// extern crate connect4;
    ///
    /// use connect4::engine::*;
    ///
    /// fn main() {
    ///     let play_col = |game: Game, col| -> Game {
    ///         match game.play(Col::b(col).unwrap()) {
    ///             Ok(game) => game,
    ///             _ => panic!("error processing move"),
    ///         }
    ///     };
    ///     let fill_col = |game, col| {
    ///         let game = play_col(game, col);
    ///         let game = play_col(game, col);
    ///         let game = play_col(game, col);
    ///         let game = play_col(game, col);
    ///         let game = play_col(game, col);
    ///         play_col(game, col)
    ///     };
    ///
    ///     let game = Game::new(Player::X);
    ///
    ///     let game = fill_col(game, 0);
    ///     let game = fill_col(game, 1);
    ///     let game = fill_col(game, 2);
    ///     let game = fill_col(game, 4);
    ///     let game = fill_col(game, 5);
    ///
    ///     assert!(game.board.get_valid_col_to_play(Col::b(0).unwrap()) == Col::b(3));
    ///     assert!(game.board.get_valid_col_to_play(Col::b(3).unwrap()) == Col::b(3));
    ///     assert!(game.board.get_valid_col_to_play(Col::b(4).unwrap()) == Col::b(6));
    ///     assert!(game.board.get_valid_col_to_play(Col::b(6).unwrap()) == Col::b(6));
    /// }
    /// ```
    ///
    pub fn get_valid_col_to_play(&self, col: Col) -> Option<Col> {
        for i in (col.0 as usize)..(NCOLS as usize) {
            if self.0[0][i] == Cell::Empty {
                return Col::b(i as u8); //  I know, I know
            }
        }
        None
    }

    /// ```rust
    /// extern crate connect4;
    ///
    /// use connect4::engine::*;
    ///
    /// fn main() {
    ///    let play_col = |game: Game, col| -> Game {
    ///        match game.play(Col::b(col).unwrap()) {
    ///            Ok(game) => game,
    ///            _ => panic!("error processing move"),
    ///        }
    ///    };
    ///    let fill_col = |game, col| {
    ///        let game = play_col(game, col);
    ///        let game = play_col(game, col);
    ///        let game = play_col(game, col);
    ///        let game = play_col(game, col);
    ///        let game = play_col(game, col);
    ///        play_col(game, col)
    ///    };
    ///
    ///    let game = Game::new(Player::X);
    ///
    ///    let game = fill_col(game, 0);
    ///    let game = fill_col(game, 1);
    ///    let game = fill_col(game, 2);
    ///    let game = fill_col(game, 4);
    ///    let game = fill_col(game, 5);
    ///
    ///    assert!(!game.board.is_valid_col_to_play(Col::b(0).unwrap()));
    ///    assert!(!game.board.is_valid_col_to_play(Col::b(1).unwrap()));
    ///    assert!(!game.board.is_valid_col_to_play(Col::b(2).unwrap()));
    ///    assert!(game.board.is_valid_col_to_play(Col::b(3).unwrap()));
    ///    assert!(!game.board.is_valid_col_to_play(Col::b(4).unwrap()));
    ///    assert!(!game.board.is_valid_col_to_play(Col::b(5).unwrap()));
    ///    assert!(game.board.is_valid_col_to_play(Col::b(6).unwrap()));
    /// }
    /// ```
    ///
    pub fn is_valid_col_to_play(&self, col: Col) -> bool {
        self.0[0][col.0 as usize] == Cell::Empty
    }

    #[must_use]
    pub fn remove_from_col(&mut self, col: Col) -> bool {
        for r in 0..NROWS {
            if self.0[r as usize][col.0 as usize] != Cell::Empty {
                self.0[r as usize][col.0 as usize] = Cell::Empty;
                return true;
            }
        }
        false
    }

    fn row_for_play(&self, col: Col) -> Option<Row> {
        for i in 0..NROWS {
            let r = NROWS - i - 1;
            if self.0[r as usize][col.0 as usize] == Cell::Empty {
                return Some(Row(r)); //  I know, I know
            }
        }
        None
    }
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.board)?;
        write!(f, "{}", self.turn)
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in 0..6 {
            for cell in self.0[row].iter() {
                write!(f, "{}", cell)?;
            }
            let _ = write!(f, "\n");
        }
        write!(f, "")
    }
}

impl std::fmt::Display for Turn {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Turn::P(player) => write!(f, "next: {}", player),
            Turn::F(Finished::Won(player)) => write!(f, "WINNER: {}", player),
            Turn::F(Finished::Draw(_player)) => write!(f, "Draw"),
        }
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Cell::P(player) => write!(f, "{}", player),
            Cell::Empty => write!(f, "_"),
        }
    }
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Player::O => write!(f, "O"),
            Player::X => write!(f, "X"),
        }
    }
}

//  ---------------------
fn empty_board() -> Board {
    Board([[Cell::Empty; NCOLS as usize]; NROWS as usize])
}

fn set_cell_on_board(mut br: Board, col: usize, row: usize, player: Cell) -> Option<Board> {
    if col < 7 && row < 6 {
        br.0[col][row] = player;
        Some(br)
    } else {
        None
    }
}

fn sline2board_row(board: Board, sline: &str, row: usize) -> Option<Board> {
    let (_, br) = sline.chars().try_fold((0, board), |(col, br), ch| {
        let br = match ch {
            '_' => Some(br),
            'O' => set_cell_on_board(br, row, col, Cell::P(Player::O)),
            'X' => set_cell_on_board(br, row, col, Cell::P(Player::X)),
            _ => None,
        }?;
        Some((col + 1, br))
    })?;

    Some(br)
}
