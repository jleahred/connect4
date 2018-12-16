use super::*;

pub(crate) fn get_best_move(game: Game) -> (Game, Option<Col>) {
    (game, Col::b(0))
}
