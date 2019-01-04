use super::*;
use crate::engine::patterns::Eval;
use crate::engine::{Turn, NCOLS};

pub fn get_best_move(game: Game, depth: u8) -> Result<(Game, Col, Eval), Game> {
    if game.moves.is_empty() {
        Ok((game, Col(3), Eval::Value(0)))
    } else {
        let (game, col, eval) = generate_step(game, depth)?;
        let eval = eval.invert();
        Ok((game, col, eval))
    }
}

struct BestMoves {
    cols: Vec<Col>,
    eval: Option<Eval>,
}

impl BestMoves {
    fn init() -> Self {
        BestMoves {
            cols: vec![],
            eval: None,
        }
    }
    fn process_move(mut self, col: Col, eval: &Eval) -> Self {
        let cmp_eval = |self_eval: &Option<Eval>, eval: &Eval| {
            if let Some(seval) = self_eval.clone() {
                seval.cmp(eval)
            } else {
                std::cmp::Ordering::Less
            }
        };
        match cmp_eval(&self.eval, eval) {
            std::cmp::Ordering::Less => {
                self.eval = Some(eval.clone());
                self.cols.clear();
                self.cols.push(col);
            }
            std::cmp::Ordering::Equal => {
                self.eval = Some(eval.clone());
                self.cols.push(col);
            }
            std::cmp::Ordering::Greater => (),
        };
        self
    }
    fn get_random_col_eval(&self) -> Option<(Col, Eval)> {
        // use rand::Rng;
        match (self.cols.len(), self.eval.clone()) {
            (0, None) => None,
            // (len, Some(eval)) => Some((self.cols[rand::thread_rng().gen_range(0, len)], eval)),
            (_len, Some(eval)) => Some((self.cols[0], eval)),
            _ => None,
        }
    }
    fn game_won(&self) -> bool {
        match self.eval {
            Some(Eval::Winner) => true,
            _ => false,
        }
    }
}

fn generate_step(game: Game, pend_steps: u8) -> Result<(Game, Col, Eval), Game> {
    let (game, bm) = {
        let mut game_bm = (game, BestMoves::init());
        for c in 0..NCOLS {
            let bm = game_bm.1;
            let col = Col(c);
            game_bm = match move_col(game_bm.0, col, pend_steps) {
                Ok((game, eval)) => {
                    // println!("{:?} {} {:?}", game.moves, col, eval);
                    (game.undo()?, bm.process_move(col, &eval))
                }
                Err(game) => (game, bm),
            };
            if game_bm.1.game_won() {
                // println!("moves__ :  {:?}", game_bm.0.moves);
                break;
            }
        }
        game_bm
    };

    match bm.get_random_col_eval() {
        Some((col, eval)) => Ok((game, col, eval.invert())),
        _ => Err(game),
    }
}

fn move_col(game: Game, col: Col, pend_steps: u8) -> Result<(Game, Eval), Game> {
    match game.play(col) {
        Ok(game) => {
            if pend_steps == 0 {
                let eval = game.eval();
                Ok((game, eval))
            } else {
                match game.turn {
                    Turn::P(_) => {
                        let (game, _col, eval) = generate_step(game, pend_steps - 1)?;
                        Ok((game, eval))
                    }
                    Turn::F(_) => {
                        let eval = game.eval();
                        Ok((game, eval))
                    }
                }
            }
        }
        Err(game) => Err(game),
    }
}
