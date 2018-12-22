extern crate connect4;
extern crate idata;
extern crate yew;

use connect4::Model;
use yew::prelude::*;

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}

// extern crate connect4;

// use connect4::engine::minmax;
// use connect4::engine::patterns::PatternsCountPlayerPonderation as PCPP;
// use connect4::engine::patterns::PatternsCountPonderation as PCP;
// use connect4::engine::*;

// fn main() {
//     let pcpp = PCPP {
//         player_current: PCP {
//             next_move_wins: 1.0,
//             imposible_avoid: 55.5,
//             vert_consecutive_hole_3inline: 0.3,
//             line3: 0.1,
//             line2: 0.01,
//             line1: 0.001,
//         },
//         player_other: PCP {
//             next_move_wins: 100.0,
//             imposible_avoid: 55.5,
//             vert_consecutive_hole_3inline: 0.3,
//             line3: 0.1,
//             line2: 0.01,
//             line1: 0.001,
//         },
//     };

//     let play_col = |game: Game, col| -> Game {
//         match game.play(Col::b(col).unwrap()) {
//             Ok(game) => game,
//             _ => panic!("error processing move"),
//         }
//     };

//     {
//         let game = Game::new(Player::O, pcpp);

//         let game = play_col(game, 3);
//         let game = play_col(game, 2);
//         let game = play_col(game, 4);
//         let game = play_col(game, 1);
//         let game = play_col(game, 5);

//         let rgame = minmax::get_best_move(game);
//         let game = match rgame {
//             Ok((game, col, eval)) => {
//                 println!("col: {:?},  eval: {:?}", col, eval);
//                 game
//             }
//             Err(game) => game,
//         };

//         println!("{}", game);
//     }
// }
