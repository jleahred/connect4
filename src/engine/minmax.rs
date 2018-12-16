use super::*;
use crate::engine::patterns::Eval;
use crate::engine::{Turn, NCOLS};

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
        let improved_val = |self_eval: &Option<Eval>, eval: &Eval| {
            if let Some(seval) = self_eval.clone() {
                seval < *eval
            } else {
                true
            }
        };

        if improved_val(&self.eval, eval) {
            self.eval = Some(eval.clone());
            self.cols.clear();
            self.cols.push(col)
        }

        self
    }
}

pub(crate) fn get_best_move(game: Game) -> Result<(Game, Col, Eval), Game> {
    Ok(generate_step(game, 1))
}

fn generate_step(game: Game, pend_steps: u8) -> (Game, Col, Eval) {
    let (game, bm) = {
        let mut game_bm = (game, BestMoves::init());
        for c in 0..NCOLS {
            let bm = game_bm.1;
            game_bm = match move_col(game_bm.0, Col(c), pend_steps) {
                Ok((game, col, eval)) => match game.undo() {
                    Ok(game) => (game, bm.process_move(col, &eval)),
                    _ => unreachable!(),
                },
                Err(game) => (game, bm),
            };
        }
        game_bm
    };

    match (bm.cols.is_empty(), bm.eval) {
        (false, Some(eval)) => (game, bm.cols[0], eval.invert()),
        _ => unreachable!(),
    }
}

fn move_col(game: Game, col: Col, pend_steps: u8) -> Result<(Game, Col, Eval), Game> {
    match game.play(col) {
        Ok(game) => {
            if pend_steps == 0 {
                let eval = game.eval();
                Ok((game, col, eval))
            } else {
                match game.turn {
                    Turn::P(_) => Ok(generate_step(game, pend_steps - 1)),
                    Turn::Won(_) => {
                        let eval = game.eval();
                        Ok((game, col, eval))
                    }
                }
            }
        }
        Err(game) => Err(game),
    }
}
