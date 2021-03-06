/// four_in_line
///
/// imposible to avoid (next_move_wins > 1)
///
/// next_move_wins (from thee_with_hole)
///
/// three_holes_consecutives_same_column
///
/// three_with_hole  ->  count & hole matrix
///
/// two with hole -> count & hole matrix
///
use super::*;

pub enum Patterns {
    FourInLine,
    P(PatternsCountPlayer),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Eval {
    Value(i64),
    Winner,
    Losser,
}

impl Eval {
    pub fn invert(&self) -> Self {
        match self {
            Eval::Winner => Eval::Losser,
            Eval::Losser => Eval::Winner,
            Eval::Value(l) => Eval::Value(-l),
        }
    }
}

impl PartialOrd for Eval {
    fn partial_cmp(&self, other: &Eval) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for Eval {
    fn cmp(&self, other: &Eval) -> std::cmp::Ordering {
        match (self, other) {
            (Eval::Winner, Eval::Winner) => std::cmp::Ordering::Equal,
            (Eval::Losser, Eval::Losser) => std::cmp::Ordering::Equal,
            (_, Eval::Winner) => std::cmp::Ordering::Less,
            (Eval::Winner, _) => std::cmp::Ordering::Greater,
            (_, Eval::Losser) => std::cmp::Ordering::Greater,
            (Eval::Losser, _) => std::cmp::Ordering::Less,
            (Eval::Value(l), Eval::Value(r)) => l.cmp(r),
        }
    }
}

impl Patterns {
    pub(crate) fn init() -> Self {
        Patterns::P(PatternsCountPlayer::init())
    }

    pub fn eval(&self, turn: Turn, pp: &PatternsCountPlayerPonderation) -> Eval {
        match turn {
            Turn::F(Finished::Won(_)) => Eval::Winner,
            Turn::F(Finished::Draw(_)) => Eval::Value(0),
            Turn::P(player) => match self {
                Patterns::FourInLine => Eval::Winner,
                Patterns::P(pcp) => Eval::Value(pcp.eval_with(player, pp)),
            },
        }
    }
}

pub struct PatternsCountPlayer {
    pub(crate) player_o: PatternsCount,
    pub(crate) player_x: PatternsCount,
    pub(crate) holes3: [[Cell; NCOLS as usize]; NROWS as usize],
}

impl PatternsCountPlayer {
    fn init() -> Self {
        PatternsCountPlayer {
            player_o: PatternsCount::init(),
            player_x: PatternsCount::init(),
            holes3: [[Cell::Empty; NCOLS as usize]; NROWS as usize],
        }
    }

    //  Eval will return the evaluation for last player played
    pub fn eval_with(&self, turn: Player, pond: &PatternsCountPlayerPonderation) -> i64 {
        let eval_player = |player: &PatternsCount, pl_pond: &PatternsCountPonderation| {
            let fval = (f64::from(player.imposible_avoid)) * pl_pond.imposible_avoid
                + (f64::from(player.line1)) * pl_pond.line1
                + (f64::from(player.line2)) * pl_pond.line2
                + (f64::from(player.line3)) * pl_pond.line3
                + (f64::from(player.next_move_wins)) * pl_pond.next_move_wins
                + (f64::from(player.vert_consecutive_hole_3inline))
                    * pl_pond.vert_consecutive_hole_3inline;
            (fval * 1_000_000.0) as i64
        };
        let (curr, other) = match turn {
            Player::O => (
                eval_player(&self.player_o, &pond.player_other),
                eval_player(&self.player_x, &pond.player_current),
            ),
            Player::X => (
                eval_player(&self.player_x, &pond.player_other),
                eval_player(&self.player_o, &pond.player_current),
            ),
        };
        (other - curr)
    }
}

#[derive(Clone, Debug)]
pub struct PatternsCountPlayerPonderation {
    pub player_current: PatternsCountPonderation,
    pub player_other: PatternsCountPonderation,
}

#[derive(Debug)]
pub struct PatternsCount {
    next_move_wins: u16,
    imposible_avoid: u16,
    vert_consecutive_hole_3inline: u16,
    line3: u16, //  a hole for win
    line2: u16, //  2 holes for win
    line1: u16, //  3 holes for win
}
impl PatternsCount {
    pub(crate) fn init() -> Self {
        PatternsCount {
            next_move_wins: 0,
            imposible_avoid: 0,
            vert_consecutive_hole_3inline: 0,
            line3: 0,
            line2: 0,
            line1: 0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct PatternsCountPonderation {
    pub next_move_wins: f64,
    pub imposible_avoid: f64,
    pub vert_consecutive_hole_3inline: f64,
    pub line3: f64,
    pub line2: f64,
    pub line1: f64,
}

pub(crate) fn get_patterns(board: &Board) -> Patterns {
    let patt = scan_horiz(board, Patterns::init());
    let patt = scan_vert(board, patt);
    let patt = scan_diag1(board, patt);
    scan_diag2(board, patt)
}

struct CellsLine([CellsCoord; 4]);

#[derive(Clone, Copy)]
struct CellsCoord {
    row: usize,
    col: usize,
}

struct ScanConf {
    startc: usize,
    limitc: usize,
    limitr: usize,
    incc: i16,
    incr: i16,
}

fn scan_horiz(board: &Board, pattern: Patterns) -> Patterns {
    scan(
        board,
        pattern,
        &ScanConf {
            startc: 0,
            limitc: (NLINE - 1) as usize,
            limitr: 0,
            incc: 1,
            incr: 0,
        },
    )
}

fn scan_vert(board: &Board, pattern: Patterns) -> Patterns {
    scan(
        board,
        pattern,
        &ScanConf {
            startc: 0,
            limitc: 0,
            limitr: (NLINE - 1) as usize,
            incc: 0,
            incr: 1,
        },
    )
}

fn scan_diag1(board: &Board, pattern: Patterns) -> Patterns {
    scan(
        board,
        pattern,
        &ScanConf {
            startc: 0,
            limitc: (NLINE - 1) as usize,
            limitr: (NLINE - 1) as usize,
            incc: 1,
            incr: 1,
        },
    )
}
fn scan_diag2(board: &Board, pattern: Patterns) -> Patterns {
    scan(
        board,
        pattern,
        &ScanConf {
            startc: (NLINE - 1) as usize,
            limitc: 0,
            limitr: (NLINE - 1) as usize,
            incc: -1,
            incr: 1,
        },
    )
}

fn scan(board: &Board, mut patterns: Patterns, sc: &ScanConf) -> Patterns {
    for c in sc.startc..(NCOLS as usize) - sc.limitc {
        for r in 0..(NROWS as usize) - sc.limitr {
            match patterns {
                Patterns::FourInLine => return patterns,
                Patterns::P(pc) => {
                    let cl = CellsLine([
                        CellsCoord {
                            row: r, // r + 0 * sc.incr,
                            col: c, // c + 0 * sc.incc,
                        },
                        CellsCoord {
                            row: (r as i16 + sc.incr) as usize, // r + 1 * sc.incr,
                            col: (c as i16 + sc.incc) as usize, // c + 1 * sc.incc,
                        },
                        CellsCoord {
                            row: (r as i16 + 2 * sc.incr) as usize,
                            col: (c as i16 + 2 * sc.incc) as usize,
                        },
                        CellsCoord {
                            row: (r as i16 + 3 * sc.incr) as usize,
                            col: (c as i16 + 3 * sc.incc) as usize,
                        },
                    ]);

                    patterns = score_4cells(&cl, pc, board);
                }
            }
        }
    }
    patterns
}

//  score_4cells  ->  vert_consecutive_hole_3inline
//  score_4line   ->  FourInLine or not
//  score_3line   ->  line3, next wins, prepare vert_consecutive_hole_3inline, Impossible avoid
//  score_2line   ->  line2,

fn score_4cells(cl: &CellsLine, patt: PatternsCountPlayer, board: &Board) -> Patterns {
    let check_impossible2avoid = |mut patt: PatternsCountPlayer| {
        let update_imp_avoid_player = |patt_player: &mut PatternsCount| {
            if patt_player.next_move_wins > 1 {
                patt_player.imposible_avoid = 1;
            }
        };
        update_imp_avoid_player(&mut patt.player_o);
        update_imp_avoid_player(&mut patt.player_x);
        patt
    };
    let check_vert_consecutive_hole_3inline = |mut patt: PatternsCountPlayer| {
        let mut count_player_o = 0;
        let mut count_player_x = 0;

        for c in 0..NCOLS as usize {
            for r in 0..(NROWS - 1) as usize {
                let cell0: Cell = patt.holes3[r][c];
                let cell1: Cell = patt.holes3[r + 1][c];
                if let (Cell::P(p0), Cell::P(p1)) = (cell0, cell1) {
                    if p0 == p1 {
                        match p0 {
                            Player::O => {
                                count_player_o += 1;
                            }
                            Player::X => {
                                count_player_x += 1;
                            }
                        }
                    }
                }
            }
        }
        patt.player_o.vert_consecutive_hole_3inline = count_player_o;
        patt.player_x.vert_consecutive_hole_3inline = count_player_x;
        patt
    };
    //  ----------
    let fpatt = score_4line(cl, patt, board);
    if let Patterns::P(patt) = fpatt {
        let patt = score_3line(cl, patt, board);
        let patt = score_2line(cl, patt, board);
        let patt = score_1line(cl, patt, board);
        let patt = check_impossible2avoid(patt);
        let patt = check_vert_consecutive_hole_3inline(patt);
        Patterns::P(patt)
    } else {
        fpatt
    }
}

fn score_4line(cl: &CellsLine, patt: PatternsCountPlayer, board: &Board) -> Patterns {
    let (count_po, count_px, _holes) = count_and_holes_4cells(cl, board);
    if count_po == 4 || count_px == 4 {
        Patterns::FourInLine
    } else {
        Patterns::P(patt)
    }
}

fn score_3line(
    cl: &CellsLine,
    mut patt: PatternsCountPlayer,
    board: &Board,
) -> PatternsCountPlayer {
    let register_hole = |cc: &CellsCoord, patt: &mut PatternsCountPlayer, player: Player| {
        if let Cell::Empty = patt.holes3[cc.row][cc.col] {
            patt.holes3[cc.row][cc.col] = Cell::P(player);
            true
        } else {
            false
        }
    };
    let get_player = |cl: &CellsLine, board: &Board| match (
        get_cell_from_coord(&cl.0[0], board),
        get_cell_from_coord(&cl.0[2], board),
    ) {
        (Cell::P(player), _) => player,
        (_, Cell::P(player)) => player,
        _ => unreachable!(),
    };
    //  ---------
    let (count_po, count_px, holes) = count_and_holes_4cells(cl, board);
    let update_3 = |player_patt: &mut PatternsCount, created_new_hole: bool| {
        player_patt.line3 += 1;
        if inmediate_cell(&Some(holes[0]), board) && created_new_hole {
            player_patt.next_move_wins += 1;
        }
    };

    let created_new_hole = match (count_po == 3 || count_px == 3, holes.is_empty()) {
        (true, false) => register_hole(&holes[0], &mut patt, get_player(cl, board)),
        _ => false,
    };
    // let created_new_hole = if count_po == 3 || count_px == 3 {
    //     if holes.len() > 0 {
    //         register_hole(&holes[0], &mut patt, get_player(cl, board))
    //     } else {
    //         false
    //     }
    // } else {
    //     false
    // };

    match (count_po, count_px) {
        (3, 0) => update_3(&mut patt.player_o, created_new_hole),
        (0, 3) => update_3(&mut patt.player_x, created_new_hole),
        _ => (),
    };

    patt
}

fn score_2line(
    cl: &CellsLine,
    mut patt: PatternsCountPlayer,
    board: &Board,
) -> PatternsCountPlayer {
    let (count_po, count_px, _holes) = count_and_holes_4cells(cl, board);

    match (count_po, count_px) {
        (2, 0) => patt.player_o.line2 += 1,
        (0, 2) => patt.player_x.line2 += 1,
        _ => (),
    };
    patt
}

fn score_1line(
    cl: &CellsLine,
    mut patt: PatternsCountPlayer,
    board: &Board,
) -> PatternsCountPlayer {
    let (count_po, count_px, _holes) = count_and_holes_4cells(cl, board);

    match (count_po, count_px) {
        (1, 0) => patt.player_o.line1 += 1,
        (0, 1) => patt.player_x.line1 += 1,
        _ => (),
    };
    patt
}

fn is_empty(cc: &CellsCoord, board: &Board) -> bool {
    match get_cell_from_coord(
        &CellsCoord {
            row: cc.row,
            col: cc.col,
        },
        board,
    ) {
        Cell::Empty => true,
        _ => false,
    }
}

fn count_and_holes_4cells(cl: &CellsLine, board: &Board) -> (u8, u8, Vec<CellsCoord>) {
    let mut count_po = 0;
    let mut count_px = 0;
    let mut holes = vec![];

    for i in 0..(NLINE as usize) {
        match get_cell_from_coord(&cl.0[i], board) {
            Cell::P(Player::O) => count_po += 1,
            Cell::P(Player::X) => count_px += 1,
            _ => holes.push(cl.0[i]),
        }
    }
    (count_po, count_px, holes)
}

fn inmediate_cell(cco: &Option<CellsCoord>, board: &Board) -> bool {
    if let Some(cc) = cco {
        if cc.row == NROWS as usize - 1 {
            true
        } else if cc.row > NROWS as usize - 1 {
            false
        } else {
            !is_empty(
                &CellsCoord {
                    row: cc.row + 1,
                    col: cc.col,
                },
                board,
            )
        }
    } else {
        false
    }
}

fn get_cell_from_coord(cc: &CellsCoord, board: &Board) -> Cell {
    board.get_cell_dangerous(cc.col, cc.row)
}
